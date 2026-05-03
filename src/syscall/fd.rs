#![allow(dead_code)]

use crate::syscall::fs::close_raw;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SocketState {
    Created,
    Listening,
    Connected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FdEntry {
    File,
    Socket {
        state: SocketState,
        parent_fd: Option<i32>,
    },
    Pipe,
}

impl FdEntry {
    pub fn label(&self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Socket {
                state: SocketState::Created,
                ..
            } => "socket(created)",
            Self::Socket {
                state: SocketState::Listening,
                ..
            } => "socket(listening)",
            Self::Socket {
                state: SocketState::Connected,
                ..
            } => "socket(connected)",
            Self::Pipe => "pipe",
        }
    }
}

pub struct FdHandle {
    fd: Option<i32>,
    entry: FdEntry,
}

impl FdHandle {
    pub fn new(fd: i32, entry: FdEntry) -> Self {
        Self {
            fd: Some(fd),
            entry,
        }
    }

    pub fn fd(&self) -> Option<i32> {
        self.fd
    }

    pub fn entry(&self) -> FdEntry {
        self.entry
    }

    fn set_entry(&mut self, entry: FdEntry) {
        self.entry = entry;
    }
}

impl Drop for FdHandle {
    fn drop(&mut self) {
        if let Some(fd) = self.fd.take() {
            let _ = close_raw(fd);
        }
    }
}

#[derive(Default)]
struct FdTable {
    entries: HashMap<i32, FdHandle>,
}

static FD_TABLE: OnceLock<Mutex<FdTable>> = OnceLock::new();

fn table() -> &'static Mutex<FdTable> {
    FD_TABLE.get_or_init(|| Mutex::new(FdTable::default()))
}

pub fn track_fd(fd: i32, entry: FdEntry) -> i32 {
    if fd < 0 {
        return fd;
    }

    let mut table = table().lock().unwrap();
    debug_assert!(
        !table.entries.contains_key(&fd),
        "attempted to track duplicate fd {fd}"
    );
    table.entries.insert(fd, FdHandle::new(fd, entry));
    fd
}

pub fn update_fd(fd: i32, entry: FdEntry) -> bool {
    let mut table = table().lock().unwrap();
    if let Some(handle) = table.entries.get_mut(&fd) {
        handle.set_entry(entry);
        true
    } else {
        false
    }
}

pub fn close_fd(fd: i32) -> bool {
    let handle = {
        let mut table = table().lock().unwrap();
        table.entries.remove(&fd)
    };

    if let Some(handle) = handle {
        drop(handle);
        true
    } else {
        false
    }
}

pub fn close_socket_cascade(fd: i32) -> Vec<i32> {
    let (closed_fds, handles) = {
        let mut table = table().lock().unwrap();

        if !table.entries.contains_key(&fd) {
            return Vec::new();
        }

        let mut seen = HashSet::new();
        let mut stack = vec![fd];
        let mut closed_fds = Vec::new();

        while let Some(current_fd) = stack.pop() {
            if !seen.insert(current_fd) {
                continue;
            }
            closed_fds.push(current_fd);

            for (entry_fd, handle) in table.entries.iter() {
                if let FdEntry::Socket {
                    parent_fd: Some(parent_fd),
                    ..
                } = handle.entry()
                {
                    if parent_fd == current_fd {
                        stack.push(*entry_fd);
                    }
                }
            }
        }

        let handles = closed_fds
            .iter()
            .filter_map(|entry_fd| table.entries.remove(entry_fd))
            .collect::<Vec<_>>();

        (closed_fds, handles)
    };

    drop(handles);
    closed_fds
}

pub fn tracked_fds() -> Vec<(i32, FdEntry)> {
    let table = table().lock().unwrap();
    table
        .entries
        .iter()
        .map(|(fd, handle)| (*fd, handle.entry()))
        .collect()
}

pub fn cleanup_fds() -> Vec<(i32, FdEntry)> {
    let handles = {
        let mut table = table().lock().unwrap();
        table.entries.drain().map(|(_, handle)| handle).collect::<Vec<_>>()
    };

    let snapshot = handles
        .iter()
        .filter_map(|handle| handle.fd().map(|fd| (fd, handle.entry())))
        .collect::<Vec<_>>();

    drop(handles);
    snapshot
}
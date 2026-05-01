#![allow(dead_code)]

use crate::syscall;

mod c;
pub mod  common;
mod cpp;
mod default_files; // no pub — only projects/ can see it
mod java;
mod js;
mod python;
mod rust;

pub fn run(parts: &[&str]) {
    match parts[0] {
        "setproject" => {
            if parts.len() < 2 {
                syscall::print("usage: setproject <lang>[=version] \n");
            } else {
                common::setup(&parts[1..]);
            }
        }
        "openproject" => {
            if parts.len() < 3 {
                syscall::print("usage: openproject <name> <lang>[=version] \n");
            } else {
                let mut path = parts[1].as_bytes().to_vec();
                path.push(b'\0');
                syscall::mkdir(path.as_ptr(), 0o755); // can read write and execute
                syscall::chdir(path.as_ptr()); //cd into the new folder
                common::setup(&parts[1..]);
            }
        }
        "run" => {
            common::run_project(parts);
        }
        _ => {}
    }
}

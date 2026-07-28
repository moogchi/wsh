// Macro for compile time conversion
// Follows x86-64 convention
// For more information read this:
// https://man7.org/linux/man-pages/man2/syscall.2.html
#![allow(dead_code)]
#[macro_export]
macro_rules! syscall {
    //1 argument
    ($nr:expr, $a1:expr) => {{
        let __nr: u64 = $nr as u64;
        let __a1: u64 = $a1 as u64;
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") __nr,
                in("rdi") __a1,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret,
                options(nostack),
            );
        }
        ret
    }};
    //2 argument
    ($nr:expr, $a1:expr, $a2:expr) => {{
        let __nr: u64 = $nr as u64;
        let __a1: u64 = $a1 as u64;
        let __a2: u64 = $a2 as u64;
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") __nr,
                in("rdi") __a1,
                in("rsi") __a2,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret,
                options(nostack),
            );
        }
        ret
    }};
    //3 argument
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {{
        let __nr: u64 = $nr as u64;
        let __a1: u64 = $a1 as u64;
        let __a2: u64 = $a2 as u64;
        let __a3: u64 = $a3 as u64;
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") __nr,
                in("rdi") __a1,
                in("rsi") __a2,
                in("rdx") __a3,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret,
                options(nostack),
            );
        }
        ret
    }};
    //4 argument
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{
        let __nr: u64 = $nr as u64;
        let __a1: u64 = $a1 as u64;
        let __a2: u64 = $a2 as u64;
        let __a3: u64 = $a3 as u64;
        let __a4: u64 = $a4 as u64;
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") __nr,
                in("rdi") __a1,
                in("rsi") __a2,
                in("rdx") __a3,
                in("r10") __a4,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret,
                options(nostack),
            );
        }
        ret
    }};
    //5 argument
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {{
        let __nr: u64 = $nr as u64;
        let __a1: u64 = $a1 as u64;
        let __a2: u64 = $a2 as u64;
        let __a3: u64 = $a3 as u64;
        let __a4: u64 = $a4 as u64;
        let __a5: u64 = $a5 as u64;
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") __nr,
                in("rdi") __a1,
                in("rsi") __a2,
                in("rdx") __a3,
                in("r10") __a4,
                in("r8") __a5,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret,
                options(nostack),
            );
        }
        ret
    }};
    //6 argument
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        let __nr: u64 = $nr as u64;
        let __a1: u64 = $a1 as u64;
        let __a2: u64 = $a2 as u64;
        let __a3: u64 = $a3 as u64;
        let __a4: u64 = $a4 as u64;
        let __a5: u64 = $a5 as u64;
        let __a6: u64 = $a6 as u64;
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") __nr,
                in("rdi") __a1,
                in("rsi") __a2,
                in("rdx") __a3,
                in("r10") __a4,
                in("r8") __a5,
                in("r9") __a6,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret,
                options(nostack),
            );
        }
        ret
    }};
}

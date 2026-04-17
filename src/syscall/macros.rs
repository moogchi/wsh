// Macro for compile time conversion
// Follows x86-64 convention
// For more information read this:
// https://man7.org/linux/man-pages/man2/syscall.2.html
#![allow(dead_code)]
#[macro_export]
macro_rules! syscall {
    //1 argument
    ($nr:expr, $a1:expr) => {{
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") $nr as u64,
                in("rdi") $a1 as u64,
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
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") $nr as u64,
                in("rdi") $a1 as u64,
                in("rsi") $a2 as u64,
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
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") $nr as u64,
                in("rdi") $a1 as u64,
                in("rsi") $a2 as u64,
                in("rdx") $a3 as u64,
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
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") $nr as u64,
                in("rdi") $a1 as u64,
                in("rsi") $a2 as u64,
                in("rdx") $a3 as u64,
                in("r10") $a4 as u64,
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
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") $nr as u64,
                in("rdi") $a1 as u64,
                in("rsi") $a2 as u64,
                in("rdx") $a3 as u64,
                in("r10") $a4 as u64,
                in("r8") $a5 as u64,
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
        let ret: i64;
        unsafe{
            std::arch::asm!(
                "syscall",
                in("rax") $nr as u64,
                in("rdi") $a1 as u64,
                in("rsi") $a2 as u64,
                in("rdx") $a3 as u64,
                in("r10") $a4 as u64,
                in("r8") $a5 as u64,
                in("r9") $a6 as u64,
                out("rcx") _,
                out("r11") _,
                lateout("rax") ret,
                options(nostack),
            );
        }
        ret
    }};
}

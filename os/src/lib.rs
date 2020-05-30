//lib.rs to add routines 
#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#[macro_use]

//rewrite std println
use core::fmt::{self,Write};
pub mod sbi;
pub mod interrupt;
pub mod init;
pub mod context;
pub mod timer;

struct Stdout;
impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.chars() {
            sbi::console_putchar(ch as u8 as usize);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}



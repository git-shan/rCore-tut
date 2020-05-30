#[macro_use]
// epxtern crate println;
 use crate::println;

use crate::sbi::set_timer;
use riscv::register::{sie, time};

pub static mut TICKS: usize = 0;

static TIMEBASE: u64 = 100000;
pub fn init() {
    unsafe {
        TICKS = 0;
        sie::set_stimer();
    }
    clock_set_next_event();
    println!("++++ setup timer!     ++++");
}

pub fn clock_set_next_event() {
    set_timer(get_cycle() + TIMEBASE);
}

fn get_cycle() -> u64 {
    time::read() as u64
}

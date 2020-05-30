
global_asm!(include_str!("boot/entry64.asm"));

pub fn init() {
    crate::interrupt::init();
    crate::timer::init();
}
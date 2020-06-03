global_asm!(include_str!("boot/entry64.asm"));

pub fn init_hw(l:usize,r:usize) {
    crate::interrupt::init();
    crate::timer::init();

    crate::memory::init(l,r);
}

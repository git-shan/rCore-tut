#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_runner)]
#![reexport_test_harness_main = "test_main"]

// #![feature(asm)]
// #![feature(global_asm)]
// global_asm!(include_str!("boot/entry64.asm"));

#[allow(unused_imports)]
use os::{self,println};
use os::init::init_hw;
use os::consts::{*};
use os::memory::{*};


#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn _start();
        fn bootstacktop();
        fn end();
    }
    #[cfg(test)]
    test_main();

    println!("_start vaddr = 0x{:x}", _start as usize);
    println!("bootstacktop vaddr = 0x{:x}", bootstacktop as usize);
    println!("hello world!");
   
    init_hw(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );

    println!(
        "free physical memory paddr = [{:#x}, {:#x})",
        end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR,
        PHYSICAL_MEMORY_END
    );
	println!(
        "free physical memory ppn = [{:#x}, {:#x})",
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
	);

    frame_allocating_test();
    panic!("you want to do nothing!");
    loop {}
}
fn frame_allocating_test() {
    println!("alloc {:x?}", alloc_frame());
    let f = alloc_frame();
    println!("alloc {:x?}", f);
    println!("alloc {:x?}", alloc_frame());
    println!("dealloc {:x?}", f);
    dealloc_frame(f.unwrap());
    println!("alloc {:x?}", alloc_frame());
    println!("alloc {:x?}", alloc_frame());
}

#[cfg(test)]
fn my_runner(tests: &[&i32]) {
    for t in tests {
        if **t == 0 {
           println!("PASSED");
        } else {
           println!("FAILED");
        }
    }
}

#[test_case]
const WILL_PASS: i32 = 0;

#[test_case]
const WILL_FAIL: i32 = 4;
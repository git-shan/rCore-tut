#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;
use os::{self,init,println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panic{:?}",info);
    loop {}
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort!");
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    init::init();
    let ostr = Some(1);
    let good:Result<i32, u8> = Ok(10);
    let bad:Result<i32, u8> = Err(0);

    println!("Hello {}: {:#?}", "rCore", ostr);
    println!("Hello {}: {:#?}", "rCore", good);
    println!("Hello {}: {:#?}", "rCore", bad);

    unsafe {
        asm!("ebreak"::::"volatile");
    }
    
    match good {
        Ok(v) => println!("working with version: {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
    } 
    match bad {
        Ok(v) => println!("working with version: {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
    } 

    unsafe {
        asm!("ebreak"::::"volatile");
    }
    panic!("end of rust_main");
    loop {}
}

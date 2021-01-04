#![feature(lang_items, core_intrinsics)]
#![feature(start)]
#![no_std]
#![no_main]

// windows entry point
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> i32 {
    main()
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

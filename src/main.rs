#![no_std]
#![no_main]

mod console;
mod thread;
mod frame_buffer;
mod game;
mod snake;
mod rand;

use game::Game;

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> i32 {
    main()
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let mut game = Game::new();

    game.run();

    0
}

#[no_mangle]
pub unsafe extern fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *((dest as usize + i) as *mut u8) = c as u8;
        i += 1;
    }
    dest
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

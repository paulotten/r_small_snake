#![no_std]
#![no_main]

extern crate libc;

extern {
    pub fn printf(format: *const u8, ...) -> i32;
}

fn write(text: &'static[u8]) {
    const MAX_SAFE_STRING_LENGTH: usize = 100;

    if text.len() > MAX_SAFE_STRING_LENGTH {
        panic!();
    }

    let mut escaped: [u8; MAX_SAFE_STRING_LENGTH+1] = [0; MAX_SAFE_STRING_LENGTH+1];

    escaped[..text.len()].clone_from_slice(text);

    unsafe {
        printf(&escaped as *const u8);
    }
}

fn clear_screen() {
    write(b"\x1B[2J");
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    write(b"before\n");
    clear_screen();
    write(b"after\n");

    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

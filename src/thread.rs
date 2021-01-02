extern crate winapi;

use winapi::shared::minwindef::DWORD;

pub fn sleep(milliseconds: u32) {
    unsafe {
        winapi::um::synchapi::Sleep(milliseconds as DWORD);
    }
}

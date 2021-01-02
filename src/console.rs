extern crate winapi;

use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::VOID;
use winapi::um::wincontypes::INPUT_RECORD;
use winapi::um::wincontypes::KEY_EVENT;
use winapi::um::wincontypes::KEY_EVENT_RECORD;
use winapi::um::wincontypes::INPUT_RECORD_Event;

pub struct Reader {
    stdin: *mut VOID
}

pub struct Writer {
    stdout: *mut VOID
}

impl Reader {
    pub fn new() -> Reader {
        // console initialization
        let stdin =
            unsafe { winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_INPUT_HANDLE) };
        let mut mode: DWORD = 0;
        let mode_ptr: *mut DWORD = &mut mode;

        unsafe {
            winapi::um::consoleapi::GetConsoleMode(stdin, mode_ptr);
            winapi::um::consoleapi::SetConsoleMode(
                stdin,
                mode
                    & !(winapi::um::wincon::ENABLE_LINE_INPUT
                        | winapi::um::wincon::ENABLE_ECHO_INPUT),
            );
        }

        Reader {
            stdin: stdin
        }
    }

    pub fn read_char(&self) -> Option<u8> {
        let mut input_record: INPUT_RECORD = INPUT_RECORD {
            EventType: 0,
            Event: unsafe { core::mem::zeroed() },
        };
        let input_record_ptr: *mut INPUT_RECORD = &mut input_record;
        let mut read: DWORD = 0;
        let read_ptr: *mut DWORD = &mut read;

        loop {
            unsafe {
                winapi::um::consoleapi::PeekConsoleInputA(
                    self.stdin,
                    input_record_ptr,
                    1,
                    read_ptr,
                );
            }

            if read == 0 {
                return None;
            }

            if input_record.EventType != KEY_EVENT {
                self.clear_event();
                continue;
            }

            let key_event_recort: KEY_EVENT_RECORD = unsafe { core::mem::transmute::<INPUT_RECORD_Event, KEY_EVENT_RECORD>(input_record.Event) };

            if key_event_recort.bKeyDown == 0 {
                self.clear_event();
                continue;
            }

            unsafe {
                let char: i8 = *key_event_recort.uChar.AsciiChar();
                self.clear_event();

                return Some(char as u8);
            }
        }
    }

    fn clear_event(&self) {
        let mut input: INPUT_RECORD = unsafe { core::mem::zeroed() };
        let input_ptr: *mut INPUT_RECORD = &mut input;
        let mut read: DWORD = 0;
        let read_ptr: *mut DWORD = &mut read;

        unsafe {
            winapi::um::consoleapi::ReadConsoleInputA(
                self.stdin,
                input_ptr,
                1,
                read_ptr,
            );
        }
    }
}

impl Writer {
    pub fn new() -> Writer {
        // console initialization
        let stdout =
            unsafe { winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE) };
        let mut mode: DWORD = 0;
        let mode_ptr: *mut DWORD = &mut mode;

        unsafe {
            winapi::um::consoleapi::GetConsoleMode(stdout, mode_ptr);
            winapi::um::consoleapi::SetConsoleMode(
                stdout,
                mode | winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            );
        }

        Writer {
            stdout: stdout
        }
    }

    pub fn write(&self, text: &[u8]) {
        unsafe {
            winapi::um::consoleapi::WriteConsoleA(
                self.stdout,
                text.as_ptr() as *const VOID,
                text.len() as DWORD,
                0 as *mut DWORD,
                0 as *mut VOID,
            );
        }
    }
}

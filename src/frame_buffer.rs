use crate::console::Writer;

pub const HEIGHT: usize = 20;
pub const WIDTH: usize = 40;
pub const AREA: usize = HEIGHT * WIDTH;

const WALL_CHAR: u8 = '#' as u8;

pub struct FrameBuffer {
    console: Writer,
    chars: [u8; AREA],
    buffer: [u8; 1],
}

impl FrameBuffer {
    pub fn get_height() -> usize {
        HEIGHT
    }

    pub fn get_width() -> usize {
        WIDTH
    }

    pub fn new() -> FrameBuffer {
        let writer = Writer::new();
        
        // disabled scrolling for height of the board
        writer.write(b"\x1B[1;23r");

        // fully clear the screen once before we start rendering to it
        writer.write(b"\x1B[2J");

        // hide the cursor
        writer.write(b"\x1B[?25l");

        FrameBuffer {
            console: writer,
            chars: [' ' as u8; AREA],
            buffer: [0],
        }
    }

    pub fn set_char(&mut self, x: usize, y: usize, c: u8) {
        self.chars[y * WIDTH + x] = c;
    }

    pub fn clear(&mut self) {
        for i in 0..AREA {
            self.chars[i] = ' ' as u8;
        }
    }

    pub fn render(&mut self) {
        // move the cursor to the top left of the screen
        self.console.write(b"\x1B[1;1H");

        self.top_bottom_bar();

        for y in 0..HEIGHT {
            self.buffer[0] = WALL_CHAR;
            self.console.write(&self.buffer);

            for x in 0..WIDTH {
                self.buffer[0] = self.chars[y * WIDTH + x];

                self.console.write(&self.buffer);
            }

            self.buffer[0] = WALL_CHAR;
            self.console.write(&self.buffer);

            self.nl();
        }

        self.top_bottom_bar();
    }

    fn top_bottom_bar(&mut self) {
        self.buffer[0] = WALL_CHAR;

        for _x in 0..WIDTH+2 {
            self.console.write(&self.buffer);
        }
        
        self.nl();
    }

    fn nl(&self) {
        self.console.write("\n".as_bytes());
    }
}

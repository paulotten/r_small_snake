use crate::frame_buffer;

const MAX_LENGTH: usize = frame_buffer::AREA;

pub struct Snake {
    // body segments
    // I'd use a tuple, but that requires an allocator to initialize an array of them
    body_x: [ i8 ; MAX_LENGTH],
    body_y: [ i8 ; MAX_LENGTH],
    // current body length
    length: usize,

    // direction velocities
    vx: i8,
    vy: i8,
}

impl Snake {
    pub fn new(x: u8, y: u8) -> Snake {
        let mut body_x = [ -1 ; MAX_LENGTH];
        let mut body_y = [ -1 ; MAX_LENGTH];
        body_x[0] = x as i8;
        body_y[0] = y as i8;

        // send the snake away from the closest side wall
        let vx: i8 = if x > frame_buffer::WIDTH as u8 / 2 { -1 } else { 1 };
        let vy: i8 = 0;

        Snake {
            body_x: body_x,
            body_y: body_y,
            length: 4,
            vx: vx,
            vy: vy,
        }
    }

    pub fn draw(&self, fb: &mut frame_buffer::FrameBuffer) {
        for i in 0..self.length {
            let c = if i == 0 { '@' } else { '#' };

            fb.set_char(self.body_x[i] as usize, self.body_y[i] as usize, c as u8);
        }
    }

    pub fn update(&mut self) {
        let head_x = self.body_x[0] + self.vx;
        let head_y = self.body_y[0] + self.vy;

        for i in (1..self.length).rev() {
            self.body_x[i] = self.body_x[i-1];
            self.body_y[i] = self.body_y[i-1];
        }

        self.body_x[0] = head_x;
        self.body_y[0] = head_y;
    }

    pub fn is_out_of_bounds(&self) -> bool {
        if self.body_x[0] < 0 || self.body_x[0] >= frame_buffer::WIDTH as i8 {
            return true;
        }

        if self.body_y[0] < 0 || self.body_y[0] >= frame_buffer::HEIGHT as i8 {
            return true;
        }

        false
    }

    // has the snake eaten its own tail?
    pub fn is_ouroboros(&self) -> bool {
        let head_x = self.body_x[0];
        let head_y = self.body_y[0];

        for i in 1..self.length {
            if self.body_x[i] == head_x && self.body_y[i] == head_y {
                return true
            }
        }

        false
    }

    pub fn grow(&mut self) {
        self.length += 2;
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_head(&self) -> (i8, i8) {
        (self.body_x[0], self.body_y[0])
    }

    pub fn set_dir_up(&mut self) {
        // no 180* turns
        // you'd immediately hit yourself
        if self.vy != 1 {
            self.vx = 0;
            self.vy = -1;
        }
    }

    pub fn set_dir_down(&mut self) {
        if self.vy != -1 {
            self.vx = 0;
            self.vy = 1;
        }
    }

    pub fn set_dir_left(&mut self) {
        if self.vx != 1 {
            self.vx = -1;
            self.vy = 0;
        }
    }

    pub fn set_dir_right(&mut self) {
        if self.vx != -1 {
            self.vx = 1;
            self.vy = 0;
        }
    }
}

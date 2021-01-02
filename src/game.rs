use crate::frame_buffer::FrameBuffer;
use crate::console::{Reader, Writer};
use crate::thread;
use crate::snake::Snake;
use crate::rand;

pub struct Game {
    fb: FrameBuffer,
    reader: Reader,
}

pub enum GameOver {
    Win,
    Lose,
}

impl Game {
    pub fn new() -> Game {
        Game {
            fb: FrameBuffer::new(),
            reader: Reader::new(),
        }
    }

    pub fn run(&mut self) {
        let mut input;

        let mut snake = Snake::new(
            rand::rand_u8(Some(FrameBuffer::get_width() as u8)),
            rand::rand_u8(Some(FrameBuffer::get_height() as u8)),
        );

        let (mut food_x, mut food_y) = self.make_food();

        let game_over_state;

        loop {
            thread::sleep(200);
            self.fb.clear();
            
            // read input
            input = match self.reader.read_char() {
                Some(c) => c,
                None => 'X' as u8,
            };
            //self.fb.set_char(0, 0, input);

            match input as char {
                'w' => snake.set_dir_up(),
                'a' => snake.set_dir_left(),
                'd' => snake.set_dir_right(),
                's' => snake.set_dir_down(),
                _ => {},
            };

            // update snake
            snake.update();

            if snake.is_out_of_bounds() || snake.is_ouroboros() {
                game_over_state = GameOver::Lose;
                break;
            }

            if snake.get_length() >= 30 {
                game_over_state = GameOver::Win;
                break;
            }

            // food check
            let (head_x, head_y) = snake.get_head();

            if head_x == food_x && head_y == food_y {
                // grow snake
                snake.grow();

                // spawn new food
                let new_food = self.make_food();

                food_x = new_food.0;
                food_y = new_food.1;
            }

            // draw snake
            snake.draw(&mut self.fb);

            // draw food
            self.fb.set_char(food_x as usize, food_y as usize, '*' as u8);

            // render frame
            self.fb.render();
        }

        let game_over_text = match game_over_state {
            GameOver::Win => "You win!",
            GameOver::Lose => "You lose.",
        };

        let w = Writer::new();
        w.write(game_over_text.as_bytes());
    }

    fn make_food(& self) -> (i8, i8) {
        let x = rand::rand_u8(Some(FrameBuffer::get_width() as u8));
        let y = rand::rand_u8(Some(FrameBuffer::get_height() as u8));

        (x as i8, y as i8)
    }
}

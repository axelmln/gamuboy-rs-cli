use std::io::{self, Write};

use crossterm::cursor;
use gamuboy::lcd::{FrameBuffer, LCD};

const PIXEL_CHAR: &str = "\u{2589}";
const DOUBLE_LEN_PIXEL_CHAR: &str = "\u{2589}\u{2589}";

const OUTPUT_FRAME_EVERY: u8 = 3;

#[derive(Clone)]
pub struct Terminal {
    y: u8,
}

impl Terminal {
    pub fn new() -> Self {
        Self { y: 0 }
    }
}

impl LCD for Terminal {
    fn draw_buffer(&mut self, frame: &FrameBuffer) {
        self.y = (self.y + 1) % OUTPUT_FRAME_EVERY;
        if self.y != 0 {
            return;
        }

        let mut stdout = io::stdout();

        crossterm::execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        let mut buffer = Vec::with_capacity(frame.len() * frame[0].len() * 20);

        for line in frame.iter() {
            let mut current_color: Option<(u8, u8, u8)> = None;
            for (j, pixel) in line.iter().enumerate() {
                if current_color != Some(*pixel) {
                    current_color = Some(*pixel);
                    write!(buffer, "\x1b[38;2;{};{};{}m", pixel.0, pixel.1, pixel.2).unwrap();
                }

                let pixel_char = if j % 2 == 0 {
                    DOUBLE_LEN_PIXEL_CHAR
                } else {
                    PIXEL_CHAR
                };
                write!(buffer, "{}", pixel_char).unwrap();
            }
            buffer.push(b'\r');
            buffer.push(b'\n');
        }

        stdout.write_all(&buffer).unwrap();

        stdout.flush().unwrap();
    }
}

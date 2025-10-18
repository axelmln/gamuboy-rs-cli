use std::env;
use std::io::stdout;
use std::process;
use std::sync::mpsc::Receiver;

use crossterm::event::KeyCode;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use crossterm::{
    cursor, execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use gamuboy::joypad::{Button, Joypad};
use gamuboy::joypad_events_handler::EventsHandler;

const PRESS_TIMEOUT: u64 = 300_000;

pub struct TermEventsHandler {
    pressed_timeout: [u64; 8],
}

impl TermEventsHandler {
    pub fn new() -> Self {
        execute!(
            stdout(),
            EnterAlternateScreen,
            cursor::Hide,
            Clear(ClearType::All)
        )
        .unwrap();
        enable_raw_mode().unwrap();

        Self {
            pressed_timeout: [0; 8],
        }
    }
}

impl TermEventsHandler {
    fn handle_event(&mut self, event: Event, joypad: &mut Joypad) {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                execute!(stdout(), LeaveAlternateScreen, cursor::Show).unwrap();
                disable_raw_mode().unwrap();

                process::exit(0);
            }

            Event::Key(KeyEvent { code, kind, .. }) => {
                let button = self.keycode_to_button(code);
                match button {
                    Some(button) => match env::consts::OS {
                        "windows" => joypad.update(button.clone(), kind == KeyEventKind::Press),
                        _ => {
                            joypad.update(button.clone(), true);
                            self.pressed_timeout[button as usize] = PRESS_TIMEOUT;
                        }
                    },
                    None => {}
                }
            }

            _ => {}
        }
    }

    fn keycode_to_button(&self, keycode: KeyCode) -> Option<Button> {
        match keycode {
            KeyCode::Char('a') => Some(Button::A),
            KeyCode::Char('b') => Some(Button::B),
            KeyCode::Enter => Some(Button::Start),
            KeyCode::Tab => Some(Button::Select),
            KeyCode::Up => Some(Button::Up),
            KeyCode::Down => Some(Button::Down),
            KeyCode::Right => Some(Button::Right),
            KeyCode::Left => Some(Button::Left),
            _ => None,
        }
    }

    fn handle_timeout(&mut self, joypad: &mut Joypad) {
        for (btn, cnt) in self.pressed_timeout.clone().iter().enumerate() {
            if *cnt == 1 {
                joypad.update(Button::from(btn as u8), false);
                self.pressed_timeout[btn] = 0;
            } else if *cnt > 0 {
                self.pressed_timeout[btn] -= 1;
            }
        }
    }
}

impl EventsHandler<Event> for TermEventsHandler {
    fn handle_events(&mut self, rx: &Receiver<Event>, joypad: &mut Joypad) {
        match env::consts::OS {
            "windows" => {}
            _ => self.handle_timeout(joypad),
        }

        let joypad_events: Vec<_> = rx.try_iter().collect();
        for evt in joypad_events {
            self.handle_event(evt, joypad);
        }
    }
}

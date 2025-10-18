use std::{
    env, fs,
    io::{stderr, Write},
    sync::mpsc,
    thread,
    time::Duration,
};

use event_handler::TermEventsHandler;
use gamuboy::{
    config::Config, gameboy::GameBoy, logger::init_logger, mode::Mode, saver::FileSaver,
};
use stereo::RodioStereo;

mod event_handler;
mod stereo;
mod term;

fn get_bool_argument(argv: &Vec<String>, flag: &str) -> bool {
    argv.into_iter()
        .find(|x| x.as_str().to_lowercase() == format!("--{}", flag))
        .is_some()
}

fn get_string_argument(argv: &Vec<String>, flag: &str) -> Option<String> {
    let index = argv
        .into_iter()
        .position(|x| x.as_str() == format!("--{}", flag));

    match index {
        Some(index) => {
            if index + 1 > argv.len() {
                return None;
            }
            Some(argv[index + 1].clone())
        }
        None => None,
    }
}

pub struct TermPoller;

impl TermPoller {
    pub fn new() -> Self {
        Self {}
    }
}

impl TermPoller {
    fn poll(&self, tx: &mpsc::Sender<crossterm::event::Event>) {
        loop {
            if let Ok(true) = crossterm::event::poll(Duration::from_micros(100)) {
                let event = crossterm::event::read().unwrap();
                tx.send(event).unwrap();
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        panic!("Rom path is required!");
    }

    let rom_path = &argv[1];

    let bootrom_path = get_string_argument(&argv, "bootrom");

    let rom = match fs::read(rom_path) {
        Ok(rom) => rom,
        Err(err) => panic!("Error occured reading rom: {}", err),
    };

    let cfg = Config {
        mode: match rom[0x143] {
            0x80 | 0xC0 => Mode::CGB,
            _ => Mode::DMG,
        },
        rom,
        headless_mode: get_bool_argument(&argv, "headless"),
        bootrom: match bootrom_path {
            Some(bootrom_path) => match fs::read(bootrom_path) {
                Ok(bootrom) => Some(bootrom),
                Err(err) => panic!("Error occured reading bootrom: {}", err),
            },
            None => None,
        },
        log_file_path: get_string_argument(&argv, "logpath"),
    };

    if cfg.headless_mode {
        _ = stderr().write_all(
            "================================== HEADLESS MODE =================================="
                .as_bytes(),
        );
    }

    init_logger(cfg.log_file_path.clone());

    let (tx, rx) = mpsc::channel();
    let joypad_poller = TermPoller::new();
    thread::spawn(move || joypad_poller.poll(&tx));

    let mut gb = GameBoy::new(
        &cfg,
        term::Terminal::new(),
        RodioStereo::new(),
        TermEventsHandler::new(),
        FileSaver::new().unwrap(),
        &rx,
    );

    gb.run();
}

use gamuboy::{
    apu::{self},
    stereo::StereoPlayer,
};
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

pub struct RodioStereo {
    sink: Option<Sink>,
    _stream: Option<OutputStream>,
}

impl RodioStereo {
    pub fn new() -> Self {
        match OutputStream::try_default() {
            Ok((_stream, stream_handle)) => {
                let sink = Sink::try_new(&stream_handle).unwrap();
                Self {
                    sink: Some(sink),
                    _stream: Some(_stream),
                }
            }
            Err(err) => {
                println!("could not open audio device: {}", err);
                Self {
                    sink: None,
                    _stream: None,
                }
            }
        }
    }
}

impl StereoPlayer for RodioStereo {
    fn play(&self, buffer: &[f32]) {
        match &self.sink {
            Some(sink) => {
                // wait for the playback to be (almost) ended
                while sink.len() > 2 {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }

                sink.append(SamplesBuffer::new(2, apu::SAMPLE_RATE, buffer));
            }
            None => {}
        }
    }
}

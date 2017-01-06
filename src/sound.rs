use std::f64::consts::PI;

use portaudio as pa;

const CHANNELS: i32 = 2;
const NUM_MILLIS: i32 = 16;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;

type Stream = pa::Stream<pa::NonBlocking, pa::Output<f32>>;

pub struct Sound {
    pa: pa::PortAudio,
    stream: Stream,
}

impl Sound {
    pub fn new() -> Sound {
        // setup output stream
        let mut sine = [0.0; TABLE_SIZE];
        for i in 0..TABLE_SIZE {
            sine[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
        }
        let mut left_phase = 0;
        let mut right_phase = 0;

        let pa = pa::PortAudio::new()
            .expect("failed to create new portaudio");

        let mut settings = pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)
            .expect("failed to create pa settings");
        settings.flags = pa::stream_flags::CLIP_OFF;

        let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
            let mut idx = 0;
            for _ in 0..frames {
                buffer[idx]   = sine[left_phase];
                buffer[idx+1] = sine[right_phase];
                left_phase += 1;
                if left_phase >= TABLE_SIZE { left_phase -= TABLE_SIZE; }
                right_phase += 3;
                if right_phase >= TABLE_SIZE { right_phase -= TABLE_SIZE; }
                idx += 2;
            }
            pa::Continue
        };

        let stream = pa.open_non_blocking_stream(settings, callback)
            .expect("failed to open pa stream");

        Sound {
            pa: pa,
            stream: stream,
        }
    }

    pub fn emit(&mut self) {
        self.stream.start().expect("failed to start pa stream");
        self.pa.sleep(NUM_MILLIS);
        self.stream.stop().expect("failed to stop pa stream");
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        self.stream.close().expect("failed to close pa stream");
    }
}


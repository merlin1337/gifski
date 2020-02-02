use crate::error::*;
use crate::source::*;
use gifski::Collector;
use std::path::PathBuf;

pub struct Lodecoder {
    frames: Vec<PathBuf>,
    fps: f32,
    durations: Vec<f64>
}

impl Lodecoder {
    pub fn new(frames: Vec<PathBuf>, fps: f32, durations: Vec<f64>) -> Self {
        Self { frames, fps, durations }
    }
}

impl Source for Lodecoder {
    fn total_frames(&self) -> u64 {
        self.frames.len() as u64
    }

    fn collect(&mut self, mut dest: Collector) -> BinResult<()> {
        let mut duration = self.durations.drain(..);
        let mut pts : f64 = 0.0;
        
        for (i, frame) in self.frames.drain(..).enumerate() {
            dest.add_frame_png_file(i, frame, pts)?;
            if let Some(delay) = duration.next() {
                pts += delay;
            } else {
                pts += 1.0 / self.fps as f64;
            }
        }
        Ok(())
    }
}

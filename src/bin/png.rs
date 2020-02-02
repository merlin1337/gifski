use crate::error::*;
use crate::source::*;
use gifski::Collector;
use std::path::PathBuf;

pub struct Lodecoder {
    frames: Vec<PathBuf>,
    fps: f32,
    durations: Vec<u16>
}

impl Lodecoder {
    pub fn new(frames: Vec<PathBuf>, fps: f32, durations: Vec<u16>) -> Self {
        Self { frames, fps, durations }
    }
}

impl Source for Lodecoder {
    fn total_frames(&self) -> u64 {
        self.frames.len() as u64
    }

    fn collect(&mut self, mut dest: Collector) -> BinResult<()> {
        let mut duration = self.durations.drain(..);
        
        for (i, frame) in self.frames.drain(..).enumerate() {
            if let Some(d) = duration.next() {
                dest.add_frame_png_file(i, frame, d)?;
            } else {
                dest.add_frame_png_file(i, frame, i as f64 / self.fps as f64)?;
            }
        }
        Ok(())
    }
}

use crate::source::*;
use gifski::Collector;
use crate::error::*;
use std::path::PathBuf;

pub struct Lodecoder {
    frames: Vec<PathBuf>,
    fps: usize,
    durations: Vec<u16>
}

impl Lodecoder {
    pub fn new(frames: Vec<PathBuf>, fps: usize, durations: Vec<u16>) -> Self {  
        Self {
            frames,
            fps,
            durations
        }
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
                let delay = ((i + 1) * 100 / self.fps) - (i * 100 / self.fps); // See telecine/pulldown.                    
                dest.add_frame_png_file(i, frame, delay as u16)?;
             }
        }
        Ok(())
    }
}

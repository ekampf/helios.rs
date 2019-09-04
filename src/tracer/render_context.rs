extern crate itertools;
extern crate time;

use itertools::Itertools;
use std::fmt;
use std::path::PathBuf;

pub struct RenderContext {
    output: PathBuf,
    width: u64,
    height: u64,
    samples: u64,

    // Some stats
    pub rays_cast: u64,
    pub start_time: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RenderTask {
    pub from_x: u64,
    pub from_y: u64,
    pub to_x: u64,
    pub to_y: u64,
}

impl RenderContext {
    pub fn new(output: PathBuf, width: u64, height: u64, samples: u64) -> RenderContext {
        RenderContext {
            output,
            width,
            height,
            samples,
            rays_cast: 0,
            start_time: time::precise_time_s(),
        }
    }

    pub fn get_tasks(&self, n: usize) -> Vec<RenderTask> {
        let mut v = Vec::with_capacity(n as usize);
        let width = self.width as usize;
        let height = self.height as usize;

        let chunk_size_w = (width as f64 / n as f64).ceil() as usize;
        let chunk_size_h = (height as f64 / n as f64).ceil() as usize;

        for mut wc in &(0..width).into_iter().chunks(chunk_size_w) {
            let from_x: u64 = wc.nth(0).unwrap() as u64;

            for mut hc in &(0..height).into_iter().chunks(chunk_size_h) {
                let from_y: u64 = hc.nth(0).unwrap() as u64;
                let to_x = (from_x + chunk_size_w as u64).min(self.width);
                let to_y = (from_y + chunk_size_h as u64).min(self.height);
                let task = RenderTask {
                    from_x,
                    from_y,
                    to_x,
                    to_y,
                };
                v.push(task);
                //                info!("RenderTask is {:#?}", task);
            }
        }

        v
    }
}

impl fmt::Debug for RenderContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RenderContext({:?}, {:?}, {:?}, {:?})",
            self.output, self.width, self.height, self.samples
        )
    }
}

extern crate rayon;

use super::tracer::*;
use indicatif::ProgressBar;
use log::info;
use rand::prelude::*;
use rayon::current_num_threads;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub fn render(
    output: PathBuf,
    width: u64,
    height: u64,
    samples: u64,
    threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let render_context = RenderContext::new(output, width, height, samples);
    info!("RenderContext is {:#?}", render_context);

    match threads {
        Some(threads_count) => {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads_count)
                .build_global()
                .unwrap();
        }
        _ => {}
    };

    info!("Using {} threads...", current_num_threads());

    let render_tasks = render_context.get_tasks(3);
    let rcm = Arc::new(Mutex::new(render_context));

    info!("Pending render tasks: {}", render_tasks.len());
    info!("Pending render tasks: {:#?}", render_tasks);
    info!("Rendering...");

    let progress_bar = ProgressBar::new(width * height);
    progress_bar.set_draw_delta(10);

    render_tasks.into_par_iter().for_each(|t| {
        // TODO: render - t.render(...)
        for _ in t.from_x..t.to_x {
            for _ in t.from_y..t.to_y {
                let mut rng = rand::thread_rng();
                let seconds = rng.gen_range(10, 200);
                let dur = time::Duration::from_millis(seconds);
                thread::sleep(dur);
                progress_bar.inc(1);
            }
        }

        let rcm = rcm.clone();
        let mut rc = rcm.lock().unwrap();

        // TODO: save rendered task data into the full render context `rc`
        // TODO: update progress
    });

    progress_bar.finish();

    Ok(())
}

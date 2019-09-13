use crate::scenes;
use crate::tracer::*;
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::current_num_threads;
use serde::*;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SceneNames {
    WeekendBalls,
    TwoSpheresPerlin,
    ThreeSpheresLight,
}

impl FromStr for SceneNames {
    type Err = serde_json::error::Error;
    fn from_str(s: &str) -> Result<SceneNames, serde_json::error::Error> {
        Ok(serde_json::from_str(&format!("\"{}\"", s))?)
    }
}

fn init_thread_pool(threads: Option<usize>) -> usize {
    match threads {
        Some(threads_count) => {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads_count)
                .build_global()
                .unwrap();
            threads_count
        }
        _ => num_cpus::get(),
    }
}

static SCENE: Emoji<'_, '_> = Emoji("🎬 ", "");
static THREAD: Emoji<'_, '_> = Emoji("🧵  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static RENDER: Emoji<'_, '_> = Emoji("🖼️  ", "");

pub fn render(
    scene_name: SceneNames,
    output: &PathBuf,
    width: u64,
    height: u64,
    samples: u64,
    threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let thread_count = init_thread_pool(threads);
    println!(
        "{} {}Initializing threadpool using {} threads...",
        style("[1/4]").bold().dim(),
        THREAD,
        current_num_threads()
    );

    println!("{} {}Loading scene...", style("[2/4]").bold().dim(), SCENE);

    let scene = match scene_name {
        SceneNames::WeekendBalls => scenes::weekend_balls::get_scene(width, height, samples),
        SceneNames::TwoSpheresPerlin => {
            scenes::two_spheres_perlin::get_scene(width, height, samples)
        }
        SceneNames::ThreeSpheresLight => {
            scenes::three_spheres_light::get_scene(width, height, samples)
        }
    };

    println!(
        "{} {}Initializing render context scene...",
        style("[3/4]").bold().dim(),
        SPARKLE
    );

    let mut render_context = RenderContext::new(width, height);

    println!(
        "{} {}Rendering image to {:?}",
        style("[4/4]").bold().dim(),
        RENDER,
        output
    );

    let progress_bar = ProgressBar::new(width * height);
    progress_bar.set_draw_delta(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {percent}% {msg}")
            .progress_chars("##-"),
    );

    render_context.render(&scene, thread_count, Some(&progress_bar));

    render_context.print_stats();

    render_context.save(output);

    let progress_bar = progress_bar.clone();
    progress_bar.finish();

    Ok(())
}

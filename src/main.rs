use std::path::PathBuf;
use structopt::StructOpt;
use log::Level;
use env_logger::Builder as LoggerBuilder;

mod noise_cmd;
mod render_cmd;
mod scenes;
mod tracer;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "example",
    about = "An example of StructOpt usage.",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
enum Cli {
    #[structopt(name = "render")]
    Render {
        #[structopt(name = "scene_name")]
        scene_name: render_cmd::SceneNames,

        #[structopt(name = "out")]
        output: PathBuf,

        #[structopt(long = "width", default_value = "300")]
        width: u64,

        #[structopt(long = "height", default_value = "200")]
        height: u64,

        #[structopt(long = "samples", default_value = "100")]
        samples: u64,

        #[structopt(long = "threads", short = "t")]
        threads: Option<usize>,

        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,

        #[structopt(long = "open", short = "o")]
        open: bool,
    },
    #[structopt(name = "noise")]
    Noise {
        #[structopt(name = "out")]
        output: PathBuf,

        #[structopt(long = "scale", short = "s", default_value = "1.5")]
        scale: f64,

        #[structopt(long = "frequency", short = "f", default_value = "1.5")]
        frequency: f64,

        #[structopt(long = "lacunarity", short = "l", default_value = "2.5")]
        lacunarity: f64,

        #[structopt(long = "persistency", short = "p", default_value = "0.5")]
        persistency: f64,

        #[structopt(long = "octaves", short = "o", default_value = "8")]
        octaves: u32,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    match args {
        Cli::Render {
            scene_name,
            output,
            width,
            height,
            samples,
            threads,
            verbose,
            open,
        } => {
            let level_filter = match verbose.log_level() {
                Some(level) => level.to_level_filter(),
                None => Level::Info.to_level_filter(),
            };
            LoggerBuilder::new().filter(None, level_filter).try_init()?;

            let result = render_cmd::render(scene_name, &output, width, height, samples, threads);

            if open {
                opener::open(output)?;
            }

            result
        }
        Cli::Noise {
            output,
            scale,
            frequency,
            lacunarity,
            persistency,
            octaves,
        } => noise_cmd::render(output, scale, frequency, lacunarity, persistency, octaves),
    }
}

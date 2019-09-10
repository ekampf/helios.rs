use std::path::PathBuf;
use structopt::StructOpt;

mod render_cmd;
mod tracer;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "example",
    about = "An example of StructOpt usage.",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
enum Cli {
    #[structopt(name = "render")]
    Render {
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    match args {
        Cli::Render {
            output,
            width,
            height,
            samples,
            threads,
            verbose,
            open,
        } => {
            let name = env!("CARGO_PKG_NAME");
            verbose.setup_env_logger(name)?;

            let result = render_cmd::render(&output, width, height, samples, threads);

            if open {
                opener::open(output)?;
            }

            result
        }
    }
}

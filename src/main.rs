use log::info;
use std::path::PathBuf;
use structopt::StructOpt;

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

        #[structopt(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    },
}

fn render(
    output: PathBuf,
    width: u64,
    height: u64,
    samples: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Render called with output {:?}, width: {}, height: {}, samples: {}",
        output, width, height, samples
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    match args {
        Cli::Render {
            output,
            width,
            height,
            samples,
            verbose,
        } => {
            let name = env!("CARGO_PKG_NAME");
            verbose.setup_env_logger(name)?;

            return render(output, width, height, samples);
        }
    }
}

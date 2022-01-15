use config::Config;
use env_logger::{fmt::Color, Builder};
use log::{debug, error, info, Level};
use std::env;
use std::process;
use structopt::StructOpt;

pub mod config;
pub mod dependency_graph;
pub mod package_json;
pub mod project;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Option<Subcommand>,

    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(StructOpt)]
enum Subcommand {
    #[structopt(about = "initialize your project")]
    Init,

    #[structopt(about = "run a command across your project")]
    Run {
        #[structopt(name = "command")]
        cmd: String,
    },
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let Cli { cmd, mut verbose } = Cli::from_args();

    verbose.set_default(Some(log::Level::Trace));

    init_logger(verbose.log_level());

    if let Err(err) = try_main(cmd) {
        error!("{}", err);
        process::exit(1);
    }

    info!("Done! ✨");
}

fn try_main(cmd: Option<Subcommand>) -> Result<()> {
    let cwd = env::current_dir().expect("Expected to find current working directory");

    debug!("Running from: {:?}", cwd);

    match cmd {
        None => {
            info!("Default command");
            let project = project::Project::find(cwd)?;

            debug!("{:?}", project);

            Ok(())
        }

        Some(Subcommand::Init) => {
            info!("Creating a new project...");

            match Config::exists(&cwd) {
                Ok(config_path) => {
                    return Err(format!("Configuration found at: {:?}", config_path).into());
                }
                Err(_) => {
                    let config_path = Config::get_config_path(&cwd);
                    Config::save(&Config::default(), &cwd)?;

                    info!("Wrote config to: {:?}", config_path);

                    Ok(())
                }
            }
        }

        Some(Subcommand::Run { cmd }) => {
            info!("Running: {:?}", cmd);

            let project = project::Project::find(cwd)?;
            let _workspace = project.workspace;

            use std::process::Command;
            let output = Command::new("yarg").output().expect("failed to run help");

            println!("{:?}", output);

            Ok(())
        }
    }
}

fn init_logger(level: Option<Level>) {
    if let Some(level) = level {
        let mut builder = Builder::from_default_env();

        builder
            .filter(None, level.to_level_filter())
            .format(move |buf, record| {
                use std::io::Write;

                let mut style = buf.style();
                let formatted = match record.level() {
                    Level::Trace => style.set_color(Color::Magenta).value("trace"),
                    Level::Debug => style.set_color(Color::Blue).value("debug"),
                    Level::Info => style.set_color(Color::Green).value("info"),
                    Level::Warn => style.set_color(Color::Yellow).value("warn"),
                    Level::Error => style.set_color(Color::Red).value("error"),
                };

                writeln!(buf, "{} {}", formatted, record.args())
            })
            .init()
    }
}

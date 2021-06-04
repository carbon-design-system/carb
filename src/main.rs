use structopt::StructOpt;
use tracing::info;

#[derive(Debug, StructOpt)]
enum Cli {
    /// Interact with the cache in your project
    Cache,

    /// Remove any generated files or folders in your project
    Clean,

    /// Run tasks associated with workspaces in your project
    Run {
        /// The name of the task to run
        task: String,
    },
}

fn main() {
    tracing_subscriber::fmt::init();

    match Cli::from_args() {
        Cli::Cache => {
            info!("Running the cache command");
        }
        Cli::Clean => {
            info!("Running the clean command");
        }
        Cli::Run { task } => {
            info!("Running the {} command", task);
        }
    };
}

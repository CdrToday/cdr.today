//! ST Commands
use crate::result::Error;
use structopt::StructOpt;

mod run;

#[derive(StructOpt)]
struct Opt {
    /// Verbose mode
    #[structopt(long, short)]
    verbose: bool,
    /// SubCommands of binary `ct`
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    command: Command,
}

#[derive(StructOpt)]
/// The server of cdr.today
enum Command {
    /// Run api server
    Run {
        /// Http server Port
        #[structopt(long, short, default_value = "1439")]
        port: u16,
    },
}

/// Exec st commands
pub async fn exec() -> Result<(), Error> {
    let opt = Opt::from_args();
    if std::env::var("RUST_LOG").is_err() {
        if opt.verbose {
            std::env::set_var("RUST_LOG", "info,ct");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    match opt.command {
        Command::Run { port } => run::exec(port).await,
    }?;

    Ok(())
}

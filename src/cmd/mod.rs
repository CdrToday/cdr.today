//! ST Commands
use crate::result::Error;
use structopt::StructOpt;

mod run;

#[derive(StructOpt)]
/// The server of cdr.today
enum Opt {
    /// Run api server
    Run {
        /// Http server Port
        #[structopt(long, short, default_value = "1439")]
        port: u16,
        /// Verbose mode
        #[structopt(long, short)]
        verbose: bool,
    },
}

/// Exec st commands
pub async fn exec() -> Result<(), Error> {
    match Opt::from_args() {
        Opt::Run { port, verbose } => run::exec(port, verbose).await,
    }?;

    Ok(())
}

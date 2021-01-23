//! CLI
use crate::result::Result;
use env_logger::{Builder, Env};
use log::info;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// If enable verbose mode
    #[structopt(short, long)]
    verbose: bool,
}

impl Opt {
    /// Execute the CLI
    pub fn exec(&self) -> Result<()> {
        self.log()?;
        Ok(())
    }

    /// Handle verbose flag
    pub fn log(&self) -> Result<()> {
        if self.verbose {
            Builder::from_env(Env::default().default_filter_or("info")).init();
            info!("verbose mode enabled");
        }
        Ok(())
    }
}

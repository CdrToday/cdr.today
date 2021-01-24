//! CLI
use crate::{actix, result::Result, Config};
use env_logger::{Builder, Env};
use log::info;
use structopt::StructOpt;

/// Command Line Entrance
#[derive(StructOpt, Debug)]
pub struct Opt {
    /// If enable verbose mode
    #[structopt(short, long)]
    verbose: bool,
}

impl Opt {
    /// Execute the CLI
    pub async fn exec(&self) -> Result<()> {
        self.init_log()?;
        actix::serve(Config::default()).await?;
        Ok(())
    }

    /// Handle verbose flag
    pub fn init_log(&self) -> Result<()> {
        if self.verbose {
            Builder::from_env(Env::default().default_filter_or("DEBUG")).init();
            info!("verbose mode enabled");
        } else {
            Builder::from_env(Env::default().default_filter_or("ERROR")).init();
        }
        Ok(())
    }
}

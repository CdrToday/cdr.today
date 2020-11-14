//! Command `run`
use crate::{api, result::Result};

/// Exec command `run`
pub async fn exec(port: u16, verbose: bool) -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        if verbose {
            std::env::set_var("RUST_LOG", "info,darwinia_shadow");
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }

    env_logger::init();
    api::serve(port).await?;
    Ok(())
}

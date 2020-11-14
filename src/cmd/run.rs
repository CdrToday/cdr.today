//! Command `run`
use crate::{api, result::Result};

/// Exec command `run`
pub async fn exec(port: u16) -> Result<()> {
    api::serve(port).await?;
    Ok(())
}

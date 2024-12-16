use anyhow::{Ok, Result};
use cli::Args;
use logging::init_logger;

mod cli;
mod errors;
mod logging;

fn main() -> Result<()> {
    let args = Args::new();
    init_logger(args.log_level.clone().into());

    let _ = args.validate()?;

    Ok(())
}

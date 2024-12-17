use anyhow::{Ok, Result};
use log::debug;
use naifuru::{cli::Args, exit_on_error, logging::init_logger};

fn main() -> Result<()> {
    let args = Args::new();

    init_logger(args.log_level.clone().into())?;
    debug!("The loglevel has been set.");

    if let Err(e) = args.validate() {
        let e_code = e.exit_code();
        e.log_errors();
        exit_on_error!(e_code);
    };

    debug!("Validation check completed");

    Ok(())
}

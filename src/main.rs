use std::fs;

use anyhow::{Context, Result};
use log::{debug, error};
use naifuru::{
    analysis_config_file::Config, bail_on_error, cli::Args, error::ErrorContext,
    logging::init_logger,
};

const DEFAULT_ERROR_EXIT_CODE: i32 = 1;

fn main() {
    if let Err(e) = run() {
        error!("{e:?}");

        if let Some(error_context) = e.downcast_ref::<ErrorContext>() {
            bail_on_error!(error_context.module.exit_code());
        }

        // ErrorContext を取得できない場合は、デフォルトのエラー コード 1 で終了します。
        bail_on_error!(DEFAULT_ERROR_EXIT_CODE);
    }
}

fn run() -> Result<()> {
    let args = Args::new();

    init_logger(args.log_level.into())?;
    debug!("The loglevel has been set.");

    args.validate()?;
    debug!("Validation check completed.");

    let config_toml_str = fs::read_to_string(&args.input_file_path).with_context(|| {
        format!(
            "Failed to read config file: {}",
            args.input_file_path.display()
        )
    })?;
    debug!("Analysis configuration file loading is complete.");

    let config: Config =
        toml::from_str(&config_toml_str).with_context(|| "Failed to parse TOML configuration")?;
    debug!("Analysis configuration file parsing is complete.");

    print!("{config:#?}");

    Ok(())
}

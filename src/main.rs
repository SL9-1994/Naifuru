use anyhow::Result;
use log::debug;
use naifuru::{
    analysis_config_file::{read_config_from_input_file, Config},
    cli::Args,
    errors::{AnalysisConfigError, CustomErrors},
    exit_on_error,
    logging::init_logger,
};

fn main() -> Result<()> {
    let args = Args::new();

    init_logger(args.log_level.into())?;
    debug!("The loglevel has been set.");

    if let Err(e) = args.validate() {
        let e_code = e.exit_code();
        e.log_errors();
        exit_on_error!(e_code);
    };
    debug!("Validation check completed.");

    let config_toml_str = match read_config_from_input_file(&args.input_file_path) {
        Ok(config) => config,
        Err(e) => {
            let err: CustomErrors = CustomErrors::from(e);
            let e_code = err.exit_code();
            err.log_errors();
            exit_on_error!(e_code);
        }
    };
    debug!("Analysis configuration file loading is complete.");

    let config: Result<Config, CustomErrors> = toml::de::from_str(config_toml_str.as_str())
        .map_err(|e| {
            CustomErrors::AnalysisConfigError(vec![AnalysisConfigError::FailedToParse(
                e.to_string(),
            )])
        });

    let _config = match config {
        Ok(config) => config,
        Err(e) => {
            let e_code = e.exit_code();
            e.log_errors();
            exit_on_error!(e_code);
        }
    };
    debug!("Analysis configuration file parsing is complete.");

    Ok(())
}

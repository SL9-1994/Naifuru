use log::{debug, error};
use naifuru::{
    analysis_config_file::{read_config_from_input_file, Config},
    bail_on_error,
    cli::Args,
    error::AppError,
    logging::init_logger,
};

const DEFAULT_ERROR_EXIT_CODE: i32 = 1;

fn main() {
    if let Err(errors) = run() {
        for error in &errors {
            error!("{}", error);
        }

        // 最初のエラーからexit_codeを決定、また、exit_codeを取得できない場合はDEFAULT_ERROR_EXIT_CODEで終了します。
        let exit_code = errors
            .first()
            .map_or(DEFAULT_ERROR_EXIT_CODE, |e| e.exit_code());

        bail_on_error!(exit_code);
    }
}

fn run() -> Result<(), Vec<AppError>> {
    let args = Args::new();

    // TODO: 非推奨: unwrap()
    init_logger(args.log_level.into()).unwrap();
    debug!("The loglevel has been set.");

    args.validate()?;
    debug!("Cli args validation check completed.");

    let config_toml_str = read_config_from_input_file(&args.input_file_path)
        .map_err(|e| vec![AppError::AnalysisConfig(e.into())])?;
    debug!("Analysis configuration file loading is complete.");

    let config: Config =
        toml::from_str(&config_toml_str).map_err(|e| vec![AppError::AnalysisConfig(e.into())])?;
    debug!("Analysis configuration file parsing is complete.");

    println!("{config:#?}");

    Ok(())
}

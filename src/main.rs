use anyhow::Result;
use log::debug;
use naifuru::{
    analyze_config_file_parser::{read_config_from_input_file, Config, Group},
    cli::Args,
    errors::{ConfigParseError, CustomErrors},
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
            CustomErrors::ConfigParseError(vec![ConfigParseError::FailedToParse(e.to_string())])
        });
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            let e_code = e.exit_code();
            e.log_errors();
            exit_on_error!(e_code);
        }
    };
    debug!("Analysis configuration file parsing is complete.");

    // Directional component grouping by g_key
    let grouped_results: Vec<Vec<&Group>> = config.group_by_key();
    println!("Grouped results:");
    for group in &grouped_results {
        println!("Group:");
        for item in group {
            println!(
                "  - path: {}, component: {:?}, g_key: {:?}",
                item.path, item.component, item.g_key
            );
        }
        println!();
    }
    println!("{:?}", config.global.config);

    Ok(())
}

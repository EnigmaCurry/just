use super::*;

/// Main entry point into `just`. Parse arguments from `args` and run. `run()`
/// will exit the proceess if `args` cannot be parsed.
#[allow(clippy::missing_errors_doc)]
pub fn run(args: impl Iterator<Item = impl Into<OsString> + Clone>) -> Result<(), i32> {
  #[cfg(windows)]
  ansi_term::enable_ansi_support().ok();

  env_logger::Builder::from_env(
    env_logger::Env::new()
      .filter("JUST_LOG")
      .write_style("JUST_LOG_STYLE"),
  )
  .try_init()
  .ok();

  let app = Config::app();

  info!("Parsing command line arguments…");
  let matches = app.get_matches_from(args);

  let config = Config::from_matches(&matches).map_err(Error::from);

  let (color, verbosity) = config
    .as_ref()
    .map(|config| (config.color, config.verbosity))
    .unwrap_or((Color::auto(), Verbosity::default()));

  let loader = Loader::new();

  config
    .and_then(|config| config.run(&loader))
    .map_err(|error| {
      if !verbosity.quiet() && error.print_message() {
        eprintln!("{}", error.color_display(color.stderr()));
      }
      error.code().unwrap_or(EXIT_FAILURE)
    })
}

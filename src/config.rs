use std::ffi::OsString;

use clap::Parser;
use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AppSettings {
  pub scraper: Scraper,
  pub proxy: Proxy,
}

#[derive(Debug, Deserialize)]
pub struct Scraper {
  pub category: String,
  pub delay: u64,
  pub limit: u32,
  pub output: String,
}

impl Default for Scraper {
  fn default() -> Self {
    Self {
      category: "https://www.amazon.com/gp/bestsellers/pc/11036071/".into(),
      delay: 0,
      limit: 1000,
      output: "bestsellers.csv".into(),
    }
  }
}

#[derive(Debug, Default, Deserialize)]
pub struct Proxy {
  pub enabled: bool,
  pub addr: String,
}

impl AppSettings {
  pub fn build_from_file(file: OsString) -> Result<Self, ConfigError> {
    let path = file.as_os_str().to_str().expect("Invalid config path");
    let builder = Config::builder()
      .add_source(File::new(path, FileFormat::Toml).required(false))
      .add_source(
        Environment::with_prefix("AMSC")
          .try_parsing(true)
          .separator("_"),
      )
      .build()?;

    builder.try_deserialize()
  }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Scrapes Amazon bestsellers page
pub struct Args {
  /// Path to configuration file
  #[arg(
    short,
    long,
    value_name = "conf",
    env = "CONF_FILE",
    default_value = "config.toml"
  )]
  pub conf: OsString,
}

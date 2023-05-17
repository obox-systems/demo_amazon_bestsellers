use std::ffi::OsString;

use clap::Parser;
use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

/// General application configuration.
#[derive(Debug, Default, Deserialize)]
pub struct AppSettings {
  /// Configuration of the scraper.
  pub scraper: Scraper,
  /// Optional proxy configuration. 
  pub proxy: Proxy,
}

/// Scraper configuration.
#[derive(Debug, Deserialize)]
pub struct Scraper {
  /// Bestsellers category to scrape.
  pub category: String,
  /// Delay between each page request.
  pub delay: u64,
  /// Limit of total store items to scrape.
  pub limit: u32,
  /// Output csv file.
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

/// Proxy configuration.
#[derive(Debug, Default, Deserialize)]
pub struct Proxy {
  /// Sets whether the proxy functionality enabled or not.
  pub enabled: bool,
  /// Proxy address.
  pub addr: String,
}

impl AppSettings {
  /// Builds application configuration from a file.
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

#[derive(Debug, Parser)]
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

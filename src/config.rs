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
  ///
  /// # Errors
  ///
  /// Returns an error if the config path contains invalid UTF-8 or if
  /// the configuration cannot be parsed.
  pub fn build_from_file(file: OsString) -> Result<Self, ConfigError> {
    let path = file
      .as_os_str()
      .to_str()
      .ok_or_else(|| ConfigError::Message("Config path contains invalid UTF-8".into()))?;
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Write;
  use tempfile::NamedTempFile;

  #[test]
  fn test_scraper_default() {
    let scraper = Scraper::default();
    assert_eq!(
      scraper.category,
      "https://www.amazon.com/gp/bestsellers/pc/11036071/"
    );
    assert_eq!(scraper.delay, 0);
    assert_eq!(scraper.limit, 1000);
    assert_eq!(scraper.output, "bestsellers.csv");
  }

  #[test]
  fn test_proxy_default() {
    let proxy = Proxy::default();
    assert!(!proxy.enabled);
    assert!(proxy.addr.is_empty());
  }

  #[test]
  fn test_app_settings_default() {
    let settings = AppSettings::default();
    assert_eq!(settings.scraper.limit, 1000);
    assert!(!settings.proxy.enabled);
  }

  #[test]
  fn test_load_from_nonexistent_file_requires_scraper() {
    // Without a file, deserialization fails because scraper fields are required
    let result = AppSettings::build_from_file("nonexistent.toml".into());
    assert!(result.is_err());
  }

  #[test]
  fn test_load_from_valid_file() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
      file,
      r#"
[scraper]
category = "https://example.com/test"
delay = 500
limit = 50
output = "test.csv"

[proxy]
enabled = true
addr = "http://proxy:8080"
"#
    )
    .unwrap();

    let result = AppSettings::build_from_file(file.path().as_os_str().to_owned());
    assert!(result.is_ok());
    let settings = result.unwrap();
    assert_eq!(settings.scraper.category, "https://example.com/test");
    assert_eq!(settings.scraper.delay, 500);
    assert_eq!(settings.scraper.limit, 50);
    assert_eq!(settings.scraper.output, "test.csv");
    assert!(settings.proxy.enabled);
    assert_eq!(settings.proxy.addr, "http://proxy:8080");
  }

  #[test]
  fn test_partial_config_requires_all_scraper_fields() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
      file,
      r#"
[scraper]
limit = 100
"#
    )
    .unwrap();

    // Partial config fails because serde requires all scraper fields
    let result = AppSettings::build_from_file(file.path().as_os_str().to_owned());
    assert!(result.is_err());
  }
}

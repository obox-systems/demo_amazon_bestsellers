//! Integration tests for amazon-bestsellers scraper.
//!
//! These tests verify the public API of the library.
//! Note: Tests requiring WebDriver are marked as `ignore` since they need
//! geckodriver running and network access.

use amazon_bestsellers::config::{AppSettings, Args, Proxy, Scraper};
use std::ffi::OsString;

#[test]
fn test_args_default_config_path() {
  // Verify Args can be parsed with default values
  let args: Args = clap::Parser::parse_from(["test"]);
  assert_eq!(args.conf, OsString::from("config.toml"));
}

#[test]
fn test_args_custom_config_path() {
  let args: Args = clap::Parser::parse_from(["test", "-c", "custom.toml"]);
  assert_eq!(args.conf, OsString::from("custom.toml"));
}

#[test]
fn test_scraper_config_defaults_are_sensible() {
  let scraper = Scraper::default();

  // Category should be a valid Amazon URL
  assert!(scraper.category.starts_with("https://www.amazon.com/"));
  assert!(scraper.category.contains("bestsellers"));

  // Delay of 0 means no delay between pages
  assert_eq!(scraper.delay, 0);

  // Default limit should be reasonable (not too small, not infinite)
  assert!(scraper.limit > 0);
  assert!(scraper.limit <= 10000);

  // Output should be a CSV file
  assert!(scraper.output.ends_with(".csv"));
}

#[test]
fn test_proxy_disabled_by_default() {
  let proxy = Proxy::default();
  assert!(!proxy.enabled);
}

#[test]
fn test_app_settings_structure() {
  let settings = AppSettings::default();

  // Verify all components are accessible
  let _scraper_category = &settings.scraper.category;
  let _proxy_enabled = settings.proxy.enabled;
}

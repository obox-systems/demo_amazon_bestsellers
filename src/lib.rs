#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! This program will scrape the title, link, rating, and number of comments from
//! the Amazon bestsellers page and save extracted data to a bestsellers.csv file.

/// Application configuration.
pub mod config;

/// Handles the Webdriver scraping process.
pub mod driver;

#[allow(missing_docs)] // Component derive macro generates undocumented impl block
/// Store item model.
pub mod item;

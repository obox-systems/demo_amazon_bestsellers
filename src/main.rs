use std::time::Duration;

use amazon_bestsellers::{
  config::{AppSettings, Args},
  driver::{fetch_all_items, find_next_page, goto_page, load_elements},
};
use anyhow::{Context, Result};
use clap::Parser;
use thirtyfour::{CapabilitiesHelper, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();
  let config =
    AppSettings::build_from_file(args.conf).context("Failed to parse configuration file")?;
  let mut caps = DesiredCapabilities::firefox();
  if config.proxy.enabled {
    caps.set_proxy(thirtyfour::Proxy::AutoConfig {
      url: config.proxy.addr,
    })?;
  }
  let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;

  let mut csv = csv::Writer::from_path(&config.scraper.output)
    .with_context(|| format!("Failed to open output file: {}", config.scraper.output))?;
  let mut page = 1u8;
  let mut total_count = 0u32;

  loop {
    goto_page(&driver, &config.scraper.category, page).await?;

    load_elements(&driver).await?;

    for item in fetch_all_items(&driver, &mut total_count)
      .await?
      .into_iter()
    {
      csv
        .serialize(item)
        .context("Failed to serialize item to CSV")?;
    }
    csv.flush()?;

    if config.scraper.limit != 0 && total_count > config.scraper.limit {
      break;
    }

    if find_next_page(&driver).await? {
      page += 1;
    } else {
      break;
    }

    tokio::time::sleep(Duration::from_millis(config.scraper.delay)).await;
  }

  driver.quit().await?;
  Ok(())
}

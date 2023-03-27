use std::time::Duration;

use amazon_bestsellers::{
  config::{AppSettings, Args},
  driver::{fetch_all_items, find_next_page, goto_page, load_elements},
};
use clap::Parser;
use thirtyfour::{prelude::*, CapabilitiesHelper};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
  let args = Args::parse();
  let config = AppSettings::build_from_file(args.conf).expect("Failed to parse config file");
  let mut caps = DesiredCapabilities::firefox();
  if config.proxy.enabled {
    caps.set_proxy(thirtyfour::Proxy::AutoConfig {
      url: config.proxy.addr,
    })?;
  }
  let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;

  let mut csv =
    csv::Writer::from_path(config.scraper.output).expect("Failed to open output csv file");
  let mut page = 1u8;
  let mut total_count = 0u32;

  loop {
    goto_page(&driver, &config.scraper.category, page).await?;

    load_elements(&driver).await?;

    for item in fetch_all_items(&driver, &mut total_count)
      .await?
      .into_iter()
    {
      csv.serialize(item).unwrap();
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

use amazon_bestsellers::driver::{fetch_all_items, goto_page, load_elements, find_next_page};
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
  let caps = DesiredCapabilities::firefox();
  let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;

  let mut csv = csv::Writer::from_path("bestsellers.csv").expect("Failed to open output csv file");
  let mut page = 1u8;
  let mut total_count = 0u32;

  loop {
    goto_page(&driver, page).await?;

    load_elements(&driver).await?;

    for item in fetch_all_items(&driver, &mut total_count)
      .await?
      .into_iter()
    {
      csv.serialize(item).unwrap();
    }

    if find_next_page(&driver).await? {
      page += 1;
    } else {
      break;
    }
  }

  csv.flush().unwrap();

  driver.quit().await?;
  Ok(())
}

use std::time::Duration;

use futures::future::try_join_all;
use thirtyfour::prelude::*;

use crate::item::{ShopItem, ShopItemComponent};

/// Scrolls through the page to load all items.
pub async fn load_elements(driver: &WebDriver) -> WebDriverResult<()> {
  for _ in 0..2 {
    driver
      .execute_async(
        r#"
        window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' });
        arguments[0]();
      "#,
        vec![],
      )
      .await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
  }

  driver
    .query(By::ClassName("a-pagination"))
    .first()
    .await?
    .scroll_into_view()
    .await
}

/// Changes window location to the specified page.
pub async fn goto_page(driver: &WebDriver, category: &str, page: u8) -> WebDriverResult<()> {
  driver.goto(format!("{}/?pg={}", category, page)).await
}

/// Iterates over all shop items on the page and returns them as a vector.
pub async fn fetch_all_items(
  driver: &WebDriver,
  total_count: &mut u32,
) -> WebDriverResult<Vec<ShopItem>> {
  let query = driver.query(By::Id("gridItemRoot"));

  if let Ok(elems) = query.all_from_selector_required().await {
    let mut items = vec![];
    for elem in elems.into_iter() {
      let item = ShopItemComponent::from(elem);
      *total_count += 1;
      println!("{}: Shop item `{}`", total_count, item.get_title().await?);
      items.push(ShopItem::from(item));
    }
    try_join_all(items).await
  } else {
    eprintln!("Failed to find shop items!");
    Ok(vec![])
  }
}

/// Checks if next page is available.
pub async fn find_next_page(driver: &WebDriver) -> WebDriverResult<bool> {
  let class_name = driver
    .query(By::ClassName("a-last"))
    .first()
    .await?
    .class_name()
    .await?;

  // If no class name or doesn't contain "a-disabled", next page is available
  Ok(!class_name.is_some_and(|c| c.contains("a-disabled")))
}

use std::time::Duration;

use futures::future::join_all;
use thirtyfour::prelude::*;

use crate::item::{ShopItem, ShopItemComponent};

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

pub async fn goto_page(driver: &WebDriver, category: &str, page: u8) -> WebDriverResult<()> {
  driver.goto({ format!("{}/?pg={}", category, page) }).await
}

pub async fn fetch_all_items(
  driver: &WebDriver,
  total_count: &mut u32,
) -> WebDriverResult<Vec<ShopItem>> {
  let query = driver.query(By::Id("gridItemRoot"));

  if let Ok(elems) = query.all_required().await {
    let mut items = vec![];
    for elem in elems.into_iter() {
      let item = ShopItemComponent::from(elem);
      *total_count += 1;
      println!("{}: Shop item `{}`", total_count, item.get_title().await?);
      items.push(ShopItem::from(item));
    }
    Ok(join_all(items).await)
  } else {
    eprintln!("Failed to find shop items!");
    Ok(vec![])
  }
}

pub async fn find_next_page(driver: &WebDriver) -> WebDriverResult<bool> {
  Ok(
    !driver
      .query(By::ClassName("a-last"))
      .first()
      .await?
      .class_name()
      .await?
      .unwrap()
      .contains("a-disabled"),
  )
}

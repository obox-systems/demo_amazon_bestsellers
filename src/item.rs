use serde::Serialize;
use thirtyfour::{
  WebElement,
  components::{Component, ElementResolver},
  prelude::*,
  resolve,
};

/// Describes shop item hooks for scraping.
///
/// Provides methods to extract product information from an Amazon item element.
#[derive(Debug, Clone, Component)]
pub struct ShopItemComponent {
  base: WebElement,
  #[by(css = "a.a-link-normal span div", single, nowait)]
  title: ElementResolver<WebElement>,
  #[by(css = "a.a-link-normal i span", single, nowait)]
  rating: ElementResolver<WebElement>,
  #[by(css = "a.a-link-normal .a-size-small", single, nowait)]
  comments: ElementResolver<WebElement>,
}

impl ShopItemComponent {
  /// Returns item's title.
  pub async fn get_title(&self) -> WebDriverResult<String> {
    resolve!(self.title).text().await
  }

  /// Returns item's link.
  pub async fn get_link(&self) -> WebDriverResult<String> {
    let path = self
      .base
      .find(By::Css("a.a-link-normal"))
      .await?
      .attr("href")
      .await?
      .unwrap_or_default();
    Ok(format!("https://www.amazon.com{path}"))
  }

  /// Returns item's rating.
  pub async fn get_rating(&self) -> WebDriverResult<String> {
    Ok(resolve!(self.rating).inner_html().await?[..3].to_owned())
  }

  /// Returns item's amount of comments.
  pub async fn get_comments(&self) -> WebDriverResult<u32> {
    Ok(
      resolve!(self.comments)
        .text()
        .await?
        .replace(',', "")
        .parse()
        .unwrap_or(0),
    )
  }
}

/// Shop item model for serialization.
#[derive(Debug, Serialize)]
pub struct ShopItem {
  title: String,
  link: String,
  rating: String,
  comments: u32,
}

impl ShopItem {
  /// Convert from shop item component to serializable shop item.
  ///
  /// # Errors
  ///
  /// Returns an error if the title or link cannot be extracted from the element.
  pub async fn from(value: ShopItemComponent) -> WebDriverResult<Self> {
    Ok(Self {
      title: value.get_title().await?,
      link: value.get_link().await?,
      rating: value.get_rating().await.unwrap_or_default(),
      comments: value.get_comments().await.unwrap_or_default(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Creates a test ShopItem for unit testing.
  fn create_test_item(title: &str, link: &str, rating: &str, comments: u32) -> ShopItem {
    ShopItem {
      title: title.to_string(),
      link: link.to_string(),
      rating: rating.to_string(),
      comments,
    }
  }

  #[test]
  fn test_shop_item_debug() {
    let item = create_test_item("Test Product", "https://amazon.com/test", "4.5", 100);
    let debug_str = format!("{:?}", item);
    assert!(debug_str.contains("Test Product"));
    assert!(debug_str.contains("4.5"));
  }

  #[test]
  fn test_shop_item_csv_serialization() {
    let item = create_test_item("Test Product", "https://amazon.com/test", "4.5", 100);

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.serialize(&item).unwrap();
    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

    assert!(data.contains("title,link,rating,comments"));
    assert!(data.contains("Test Product"));
    assert!(data.contains("https://amazon.com/test"));
    assert!(data.contains("4.5"));
    assert!(data.contains("100"));
  }

  #[test]
  fn test_shop_item_special_characters() {
    let item = create_test_item(
      "Product with \"quotes\" and, commas",
      "https://amazon.com/dp/B123",
      "3.9",
      5000,
    );

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.serialize(&item).unwrap();
    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

    // CSV should properly escape quotes and commas
    assert!(data.contains("quotes"));
    assert!(data.contains("5000"));
  }

  #[test]
  fn test_shop_item_empty_rating() {
    let item = create_test_item("Product", "https://amazon.com/test", "", 0);

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.serialize(&item).unwrap();
    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

    // Should handle empty rating gracefully
    assert!(data.contains("Product"));
  }
}

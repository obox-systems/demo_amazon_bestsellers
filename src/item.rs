use serde::Serialize;
use thirtyfour::{
  components::{Component, ElementResolver},
  prelude::*,
  resolve, WebElement,
};

/// Describes shop item hooks for scraping.
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
      .unwrap();
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
        .unwrap(),
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
  pub async fn from(value: ShopItemComponent) -> Self {
    Self {
      title: value.get_title().await.unwrap(),
      link: value.get_link().await.unwrap(),
      rating: value.get_rating().await.unwrap_or_default(),
      comments: value.get_comments().await.unwrap_or_default(),
    }
  }
}

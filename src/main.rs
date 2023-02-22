use std::time::Duration;

use serde::Serialize;
use thirtyfour::{prelude::*, components::{ElementResolver, Component}};

#[derive(Debug, Clone, Component)]
pub struct ShopItemComponent {
  base: WebElement,
  #[by(css = "a.a-link-normal span div",  single)]
  title: ElementResolver<WebElement>,
  #[by(css = "a.a-link-normal i span", single)]
  rating: ElementResolver<WebElement>,
  #[by(css = "a.a-link-normal .a-size-small", single)]
  comments: ElementResolver<WebElement>
}

impl ShopItemComponent {
  pub async fn get_title(&self) -> WebDriverResult<String> {
    self.title.resolve().await?.text().await
  }

  pub async fn get_link(&self) -> WebDriverResult<String> {
    let path = self.base.find(By::Css("a.a-link-normal"))
      .await?.attr("href").await?.unwrap();
    Ok(format!("https://www.amazon.com{path}"))
  }

  pub async fn get_rating(&self) -> WebDriverResult<String> {
    Ok(self.rating.resolve().await?.inner_html().await?[..3].to_owned())
  }

  pub async fn get_comments(&self) -> WebDriverResult<u32> {
    Ok(self.comments.resolve().await?.text().await?.replace(",", "").parse().unwrap())
  }
}

#[derive(Serialize)]
pub struct ShopItem {
  title: String,
  link: String,
  rating: String,
  comments: u32
}

impl ShopItem {
  pub async fn from(value: ShopItemComponent) -> Self {
    Self {
      title: value.get_title().await.unwrap(),
      link: value.get_link().await.unwrap(),
      rating: value.get_rating().await.unwrap(),
      comments: value.get_comments().await.unwrap()
    }
  }
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
  let caps = DesiredCapabilities::firefox();
  let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;
  driver.goto("https://www.amazon.com/gp/bestsellers/fashion/1294868011/").await?;

  for _ in 0..2 {
    driver.execute_async(r#"
        window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' });
        arguments[0]();
      "#,
      vec![]
    ).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
  }

  driver.query(By::ClassName("a-pagination")).first().await?.scroll_into_view().await?;


  let query = driver.query(By::Id("gridItemRoot"));

  let mut csv = csv::Writer::from_path("bestsellers.csv")
    .expect("Failed to open output csv file");
  if let Ok(elems) = query.all_required().await {
    for (i, elem) in elems.into_iter().enumerate() {
      let item = ShopItemComponent::from(elem);
      println!("{}: Shop item `{}`", i + 1, item.get_title().await?);
      csv.serialize(ShopItem::from(item).await).unwrap();
    }
  } else {
    eprintln!("Failed to find shop items!");
  }

  csv.flush().unwrap();

  driver.quit().await?;
  Ok(())
}

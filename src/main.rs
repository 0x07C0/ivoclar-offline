use std::{error::Error, path::PathBuf};

use thirtyfour::{DesiredCapabilities, WebDriver, By};
use tokio::fs;

async fn save_page(driver: &WebDriver, website: &str, path: &str) -> Result<(), Box<dyn Error>> {
  driver.goto(format!("{website}/{path}")).await?;
  
  fs::create_dir_all(format!("page/{path}")).await?;
  let source = driver.source().await?;
  fs::write(format!("page/{path}/index.html"), source.as_bytes()).await?;
  for img in driver.find_all(By::Css("img")).await? {
    let src = img.attr("src").await?;
    if let Some(src) = src {
      if let Ok(bytes) = reqwest::get(format!("{website}/{src}")).await?.bytes().await {
        let path = format!("page/{src}");
        let path = PathBuf::from(path);
        fs::create_dir_all(&path.parent().unwrap()).await?;
        fs::write(&path, bytes).await?;
      }
    }
  }

  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let caps = DesiredCapabilities::firefox();
  let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;
  let website = "https://www.ivoclar.com";
  let init_path = "en_us/ids"; 

  save_page(&driver, website, init_path).await?;

  for page in vec![
    "workflows",
    "product-highlights"
  ] {
    save_page(&driver, website, &format!("{init_path}/{page}")).await?
  }

  driver.quit().await?;
  Ok(())
}

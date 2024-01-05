use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("headless")?;
    caps.add_chrome_arg("disable-gpu")?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://exchange.coinbase.com/markets").await?;
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let rows = driver
        .find_all(By::XPath("//a[contains(@href, '/trade/')]"))
        .await?;

    println!("Market Data from Coinbase.com");
    for (index, row) in rows.into_iter().enumerate() {
        if index >= 6 {
            break;
        }

        let row_text = row.text().await?;
        let mut lines = row_text.lines();

        if let (Some(token), Some(_), Some(price)) = (lines.next(), lines.next(), lines.next()) {
            println!("Token: {}", token);
            println!("Price: $ {}", price);
            println!("---");
        }
    }

    driver.quit().await?;

    Ok(())
}

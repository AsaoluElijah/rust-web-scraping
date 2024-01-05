use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::edge();
    // caps.add_chrome_arg("headless")?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://exchange.coinbase.com/markets").await?;
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    // Select all rows in the cryptocurrency table
    let rows = driver
        .find_all(By::XPath("//a[contains(@href, '/trade/')]"))
        .await?;

    for (index, row) in rows.into_iter().enumerate() {
        if index >= 5 {
            break; // Limit to first five entries
        }

        // Extracting text from the entire row
        let row_text = row.text().await?;

        // Process the row text to extract the cryptocurrency name and price
        // Implement processing based on the observed structure of the row text
        println!("Row {}: {}", index + 1, row_text);
    }

    driver.quit().await?;

    Ok(())
}

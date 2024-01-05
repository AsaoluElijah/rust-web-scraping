use thirtyfour::prelude::*;
use thirtyfour::Key;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();

    caps.add_chrome_arg("headless")?;
    caps.add_chrome_arg("disable-gpu")?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to Google.
    driver.goto("https://www.google.com").await?;

    // Find the search box using its name attribute 'q'.
    let search_box = driver.find(By::Name("q")).await?;

    // Type 'rust' into the search box.
    search_box.send_keys("rust").await?;

    // Submit the search by simulating an Enter key press.
    search_box.send_keys(Key::Enter.to_string()).await?;
    // send_keys(Keys::Enter).await?;

    // Wait for the search results to load.
    // This is a simple delay for demonstration purposes.
    // In a real-world scenario, you should use more robust waiting methods.
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Optionally, you can check the title or any other element to ensure that
    // the search results page has loaded.
    // For example:
    // assert_eq!(driver.title().await?, "rust - Google Search");

    // Get the source of the page.
    let page_source = driver.source().await?;

    // Print the page source.
    println!("{}", page_source);

    // Close the browser.
    driver.quit().await?;

    Ok(())
}

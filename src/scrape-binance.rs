use reqwest::get;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.binance.com/en/markets/overview";
    let body = get(url).await?.text().await?;

    let document = Html::parse_document(&body);
    let token_selector = Selector::parse(".css-vlibs4").unwrap();

    // Counter to track the top 5 tokens
    let mut count = 0;

    println!("Market Data from Binance.com");

    for token_element in document.select(&token_selector) {
        // Select the elements containing the token name and price
        let token_name_selector = Selector::parse(".subtitle3").unwrap();
        let token_price_selector = Selector::parse(".body2").unwrap();

        // Find the token name and price within the token element
        let token_name = token_element
            .select(&token_name_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join(" ");
        let token_price = token_element
            .select(&token_price_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join(" ");

        // Print the token name and price
        println!("Token: {}", token_name);
        println!("Price: {}", token_price);
        println!("---");

        count += 1;

        // Break the loop when the top 5 tokens have been processed
        if count >= 5 {
            break;
        }
    }

    Ok(())
}

use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    // Web scraping using RUST
    let client = Client::new();

    let response = client.get("https://scrapeme.live/shop/")
        .send().await.unwrap();

    let html_content = response.text().await.unwrap();

    let document = Html::parse_document(&html_content);

    let html_product_selector = Selector::parse("li.product").unwrap();
    let html_products = document.select(&html_product_selector);

    for product in html_products {
        let url = product
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);

        let image_url = product
            .select(&Selector::parse("img").unwrap())
            .next()
            .and_then(|a| a.value().attr("src"))
            .map(str::to_owned);

        let product_name = product
            .select(&Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());

        let product_price = product
            .select(&Selector::parse(".price").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());

        println!("name = {:?}, price = {:?}, url = {:?}, image_url = {:?}",
            product_name, product_price, url, image_url);
    }
}

use scraper::{Html, Selector};
use std::error::Error;
use tokio;

// Get the result of the oil price and print it to std output
// fn get_html(doc_body: &Html, selector: &Selector, desc: &str) {
//     for title in doc_body.select(&selector) {
//         let titles = title.text().collect::<Vec<_>>();
//         println!("{}: {}USD", desc, titles[0])
//     }
// }

async fn get_html_async(
    doc_body: &Html,
    selector: &Selector,
    desc: &str,
) -> Result<(), Box<dyn Error>> {
    for title in doc_body.select(&selector) {
        let titles = title.text().collect::<Vec<_>>();
        println!("{}: {}USD", desc, titles[0])
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // fn main() {
    // let response = reqwest::blocking::get("https://oilprice.com")
    //     .unwrap()
    //     .text()
    //     .unwrap();
    let response = reqwest::get("https://oilprice.com").await?.text().await?;
    let doc_body = Html::parse_document(&response);
    let wti = Selector::parse("tr[data-hash='WTI-Crude'] td.value").unwrap();
    let wti_change = Selector::parse("tr[data-hash='WTI-Crude'] td.change_percent").unwrap();
    let brent = Selector::parse("tr[data-hash='Brent-Crude'] td.value").unwrap();
    let brent_change = Selector::parse("tr[data-hash='Brent-Crude'] td.change_percent").unwrap();

    // Without concurency
    // get_html(&doc_body, &wti, "WTI price");
    // get_html(&doc_body, &wti_change, "WTI change");
    // get_html(&doc_body, &brent, "Brent price");

    let task1 = get_html_async(&doc_body, &wti, "WTI price");
    let task2 = get_html_async(&doc_body, &wti_change, "WTI change");
    let task3 = get_html_async(&doc_body, &brent, "Brent price");
    let task4 = get_html_async(&doc_body, &brent_change, "Brent change");

    tokio::try_join!(task1, task2, task3, task4)?;
    Ok(())
}

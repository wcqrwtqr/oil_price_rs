use colored::*;
use scraper::{Html, Selector};
use std::error::Error;
use tokio;

async fn get_html_async(
    doc_body: &Html,
    selector: &Selector,
    desc: &str,
    is_value: bool,
) -> Result<(), Box<dyn Error>> {
    for title in doc_body.select(&selector) {
        let titles = title.text().collect::<Vec<_>>();

        // Check the condition if we are fetching values or change
        if is_value {
            // Parse the values as float after trim whitespace
            let val: Result<f64, _> = titles[0].trim().parse();
            match val {
                Ok(x) if x > 90.0 => println!("{}: {} USD", desc, format!("{}", x).green()),
                Ok(x) if x > 70.0 && x < 90.0 => {
                    println!("{}: {} USD", desc, format!("{}", x).yellow())
                }
                Ok(x) if x < 70.0 => println!("{}: {} USD", desc, format!("{}", x).red()),
                _ => println!("{}: {} USD", desc, titles[0]),
            }
        } else {
            // If we choose false then we will search for the + or - in the change
            let val: &str = titles[0];
            match val {
                x if x.contains("+") => println!("{}: {}", desc, format!("{}", x).green()),
                x if x.contains("-") => println!("{}: {}", desc, format!("{}", x).red()),
                _ => println!("{}: {}", desc, val),
            }
        }
    }
    Ok(())
}

// Old function with no text coloring based on prices
// async fn get_html_async(
//     doc_body: &Html,
//     selector: &Selector,
//     desc: &str,
// ) -> Result<(), Box<dyn Error>> {
//     for title in doc_body.select(&selector) {
//         let titles = title.text().collect::<Vec<_>>();
//         println!("{}: {}USD", desc, titles[0])
//     }
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Url response
    let response = reqwest::get("https://oilprice.com").await?.text().await?;
    let doc_body = Html::parse_document(&response);

    // Selectors
    let wti = Selector::parse("tr[data-hash='WTI-Crude'] td.value").unwrap();
    let wti_change = Selector::parse("tr[data-hash='WTI-Crude'] td.change_percent").unwrap();
    let brent = Selector::parse("tr[data-hash='Brent-Crude'] td.value").unwrap();
    let brent_change = Selector::parse("tr[data-hash='Brent-Crude'] td.change_percent").unwrap();

    // Tasks
    let task1 = get_html_async(&doc_body, &wti, "WTI price", true);
    let task2 = get_html_async(&doc_body, &wti_change, "WTI change", false);
    let task3 = get_html_async(&doc_body, &brent, "Brent price", true);
    let task4 = get_html_async(&doc_body, &brent_change, "Brent change", false);
    // let task1 = get_html_async(&doc_body, &wti, "WTI price");
    // let task2 = get_html_async(&doc_body, &wti_change, "WTI change");
    // let task3 = get_html_async(&doc_body, &brent, "Brent price");
    // let task4 = get_html_async(&doc_body, &brent_change, "Brent change");

    tokio::try_join!(task1, task2, task3, task4)?;
    Ok(())
}

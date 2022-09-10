mod card;

use std::collections::HashMap;
use once_cell::sync::Lazy;
use reqwest::{Client, header::{self, HeaderMap, HeaderValue}};
use scraper::{Html, Selector};

pub use self::card::Card;
pub use reqwest::Result;

static ROOT_URL: &'static str = "https://en.shindanmaker.com";
// The client to interact with shindanmaker
static CLIENT: Lazy<Client> = Lazy::new(||{
    let mut headers = HeaderMap::with_capacity(1);
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded")
    );

    Client::builder()
        .cookie_store(true)
        .user_agent(
            "Mozilla/5.0 (X11; Linux x86_64; rv:104.0) \
             Gecko/20100101 Firefox/104.0")
        .default_headers(headers)
        .build()
        .unwrap()
});

/// Get the diagnosis information from [Persona Generator].
///
/// [Persona Generator]: https://en.shindanmaker.com/917962
pub async fn get_persona(name: &str) -> Result<Card> {
    let content = fetch_diagnosis(917962, name).await?;

    let fragment = Html::parse_document(&content);
    let selector = Selector::parse("span").unwrap();

    // initialize info_list
    let mut info_list = Vec::new();

    // traverse the list achieved by the selector
    for element in fragment.select(&selector) {
        // filter the text of the element with id shindanName
        if let Some("shindanResult") = element.value().attr("id") {
            // insert item into list
            for (index, item) in element.text().enumerate() {
                if index == 0 || index == 6 {
                    info_list.push(item);
                } else {
                    info_list.push(item.split(":").last().unwrap());
                }
            }

            break;
        }
    }

    // the simplest style
    Ok(Card {
        name: name.to_owned(),
        sex: info_list[1].to_owned(),
        race: info_list[2].to_owned(),
        character: info_list[3].to_owned(),
        talent: info_list[4].to_owned(),
        camp: info_list[5].to_owned(),
        hobby: info_list[6].to_owned(),
        hair: info_list[7].to_owned(),
        pupil: info_list[8].to_owned(),
        danger: info_list[9].to_owned(),
        lucky: info_list[10].to_owned(),
    })
}

/// Get the diagnosis information as string from `https://en.shindanmaker.com/:page_id`
pub async fn get_by_id(page_id: u64, name: &str) -> Result<String> {
    let content = fetch_diagnosis(page_id, name).await?;

    let fragment = Html::parse_document(&content);
    let selector = Selector::parse("span").unwrap();

    // traverse the list achieved by the selector
    for element in fragment.select(&selector) {
        // filter the text of the element with id shindanName
        if let Some("shindanResult") = element.value().attr("id") {
            return Ok(element
                        .text()
                        .collect::<Vec<&str>>()
                        .join("\n"));
        }
    }

    unreachable!()
}


// Fetch the diagnosis result as raw html text
async fn fetch_diagnosis(page_id: u64, name: &str) -> Result<String> {
    let resp = CLIENT.get(format!("{}/{}", ROOT_URL, page_id))
        .send()
        .await?
        .error_for_status()? // Page doesn't exist
        .text()
        .await?;

    let mut args = HashMap::with_capacity(3);
    args.insert("shindanName", name.to_owned());

    // Send Approximation
    // https://rust-lang.github.io/async-book/07_workarounds/03_send_approximation.html
    {
        // achieve html fragment and selector
        let fragment = Html::parse_document(&resp);
        let selector = Selector::parse("input").unwrap();

        // traverse the list achieved by the selector
        for element in fragment.select(&selector) {
            let element = element.value();

            match (element.attr("name"), element.attr("value")) {
                (Some("_token"), Some(token)) => {
                    args.insert("_token", token.to_owned());
                },

                (Some("hiddenName"), Some(hidden_name)) => {
                    args.insert("hiddenName", hidden_name.to_owned());
                },

                _ => continue,
            }
        }
    }

    let resp = CLIENT.post(format!("{ROOT_URL}/{page_id}"))
        .json(&args)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(resp)
}

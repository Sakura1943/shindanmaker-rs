mod card;

use once_cell::sync::Lazy;
use reqwest::{
    header::{self, HeaderMap, HeaderValue, USER_AGENT},
    Client,
};
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;

pub use self::card::Card;
pub use reqwest::Result;

static ROOT_URL: &str = "https://en.shindanmaker.com";
// The client to interact with shindanmaker
static CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = HeaderMap::with_capacity(1);
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    Client::builder()
        .cookie_store(true)
        .user_agent(USER_AGENT)
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

    let element: Vec<ElementRef> = fragment
        .select(&selector)
        .filter(|el| el.value().attr("id") == Some("shindanResult"))
        .collect();

    let info_list = element[0]
        .text()
        .enumerate()
        .map(|(index, item)| {
            if index == 0 || index == 6 {
                item
            } else {
                item.split(':').last().unwrap()
            }
        })
        .collect::<Vec<&str>>();

    // the simplest style
    Ok(Card {
        name: info_list[0].to_owned(),
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
    let element: Vec<ElementRef> = fragment
        .select(&selector)
        .filter(|el| el.value().attr("id") == Some("shindanResult"))
        .collect();

    Ok(element[0].text().collect::<Vec<&str>>().join("\n"))
}

// Fetch the diagnosis result as raw html text
async fn fetch_diagnosis(page_id: u64, name: &str) -> Result<String> {
    let resp = CLIENT
        .get(format!("{}/{}", ROOT_URL, page_id))
        .send()
        .await?
        .error_for_status()? // Page doesn't exist
        .text()
        .await?;

    let mut args = HashMap::with_capacity(5);
    args.insert("type", "name".to_owned());
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
                (Some("shindan_token"), Some(shindan_token)) => {
                    args.insert("shindan_token", shindan_token.to_owned());
                }

                (Some("hiddenName"), Some(hidden_name)) => {
                    args.insert("hiddenName", hidden_name.to_owned());
                }

                (Some("_token"), Some(token)) => {
                    args.insert("_token", token.to_owned());
                }

                _ => continue,
            }
        }
    }

    let resp = CLIENT
        .post(format!("{ROOT_URL}/{page_id}"))
        .json(&args)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(resp)
}

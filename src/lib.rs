mod card;

use std::collections::HashMap;
use reqwest::{Client, header::{self, HeaderMap, HeaderValue}};
use scraper::{Html, Selector};

pub use self::card::Card;
pub use reqwest::Result;

static ROOT_URL: &'static str = "https://en.shindanmaker.com";

/// Fetch diagnosis information from `https://en.shindanmaker.com/917962` .
pub async fn get(name: &str) -> Result<Card> {
    // achieve html fragment and selector
    let (fragment, selector) = common_func(917962, name).await?;

    // initialize info_list
    let mut info_list = Vec::new();
    let mut index = 0;

    // traverse the list achieved by the selector
    for element in fragment.select(&selector) {
        // filter the text of the element with id shindanName
        if let Some("shindanResult") = element.value().attr("id") {
            // insert item into list
            for item in element.text() {
                if index == 0 || index == 6 {
                    info_list.push(item);
                } else {
                    info_list.push(item.split(":").last().unwrap());
                }

                index += 1;
            }
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

// get else page
pub async fn get_else(page_id: i64, name: &str) -> Result<String> {
    // achieve html fragment and selector
    let (fragment, selector) = common_func(page_id, name).await?;
    let mut info = String::new();
    let mut index = 0;
    // traverse the list achieved by the selector
    for element in fragment.select(&selector) {
        // filter the text of the element with id shindanName
        if let Some("shindanResult") = element.value().attr("id") {
            // insert item into list
            for item in element.text() {
                if index == element.text().count() - 1 {
                    info += &(item.to_owned());
                } else {
                    info += &(item.to_owned() + "\n");
                }
                index += 1;
            }
        }
    }

    Ok(info)
}


// Common part
async fn common_func(page_id: i64, name: &str) -> Result<(Html, Selector)> {
    let mut headers = HeaderMap::with_capacity(1);
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded")
    );

    // create reqwest client
    let client = Client::builder()
        .cookie_store(true)
        .user_agent(
            "Mozilla/5.0 (X11; Linux x86_64; rv:104.0) \
             Gecko/20100101 Firefox/104.0")
        .default_headers(headers)
        .build()
        .unwrap();

    // achieve html content (It is used to achieve token and hiddenName) (Method: get)
    let resp = client.get(format!("{}/{}", ROOT_URL, page_id))
        .send()
        .await?
        .text()
        .await?;

    // achieve html fragment and selector
    let fragment = Html::parse_document(&resp);
    let selector = Selector::parse("input").unwrap();

    let mut args = HashMap::with_capacity(3);
    args.insert("shindanName", name);

    // traverse the list achieved by the selector
    for element in fragment.select(&selector) {
        let element = element.value();

        match (element.attr("name"), element.attr("value")) {
            (Some("_token"), Some(token)) => {
                args.insert("_token", token);
            },

            (Some("hiddenName"), Some(hidden_name)) => {
                args.insert("hiddenName", hidden_name);
            },

            _ => continue,
        }
    }
    // achieve html content (It is used to achieve result) (Method: post)
    let resp = client.post(format!("{}/{}", ROOT_URL, page_id))
        .json(&args)
        .send()
        .await?
        .text()
        .await?;

    // return html fragment and selector
    Ok((
        Html::parse_document(&resp),
        Selector::parse("span").unwrap()
    ))
}

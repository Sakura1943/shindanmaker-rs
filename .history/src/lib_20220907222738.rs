pub mod error;
pub mod utils;
use std::collections::HashMap;
use reqwest::header::HeaderMap;
use scraper::{Html, Selector};
use error::Error;
use utils::Datas;

/// # The method to achieve information from api
/// *parameters:* name: `&str` <br>
/// *return:* `anyhow::Result<String>` (the result from this api)
/// ## example:
/// *if return anyhow::Result:*
/// ```rust
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///   let result = shindanmaker_rs::get("demo")?;
///   println!("{}", result);
///   Ok(())
/// }
/// ```
/// *if return else or none:*
/// ```rust
/// #[tokio::main]
/// async fn main() {
///   let result = shindanmaker_rs::get("demo").unwrap();
///   println!("{}", result);
/// }
/// ```
///
pub async fn get<'a>(name: &str) -> anyhow::Result<Datas<&str>> {
    // TODO: create reqwest client
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;

    // TODO: define headers and json data to be sent (Type: HashMap)
    let mut headers = HeaderMap::new();
    let mut json_data = HashMap::new();

    // TODO: insert header information
    headers.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0".parse()?);
    headers.insert("content-type", "application/x-www-form-urlencoded".parse()?);

    // TODO: achieve html content (It is used to achieve token and hiddenName) (Method: get)
    let res = client.get("https://en.shindanmaker.com/917962")
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;

    // TODO: achieve html fragment and selector
    let fragment = Html::parse_document(&res);
    let selector = Selector::parse("input").map_err(|_| Error::GetElementTextErr)?;

    // TODO: initialize token and hidden_name(Type: String)
    let mut token = String::new();
    let mut hidden_name = String::new();

    // TODO: traverse the list achieved by the selector
    for item in fragment.select(&selector) {
        // TODO: filter elements named token and name
        if item.value().attr("name") == Some("_token") {
            // TODO: achieve _token
            if let Some(_token) = item.value().attr("value") {
                // TODO: assign _token to token
                token = _token.to_owned();
            }
        } else if item.value().attr("name") == Some("hiddenName") {
            // TODO: achieve hiddenName
            if let Some(_hidden_name) = item.value().attr("value") {
                // TODO: assign _hidden_name to hidden_name
                hidden_name = _hidden_name.to_owned();
            }
        }
    }

    // TODO; insert _token, shindanName and hiddenName into json_data(Type: HashMap)
    json_data.insert("_token", token.as_str());
    json_data.insert("shindanName", name);
    json_data.insert("hiddenName", hidden_name.as_str());

    // TODO: achieve html content (It is used to achieve result) (Method: post)
    let res = client.post("https://en.shindanmaker.com/917962")
        .headers(headers)
        .json(&json_data)
        .send()
        .await?
        .text()
        .await?;

    // TODO: achieve html fragment and selector
    let fragment = Html::parse_document(&res);
    let selector = Selector::parse("span").map_err(|_| Error::GetElementTextErr)?;

    // TODO: initialize info_list(Type Vec<String>)
    let mut info_list: Vec<String> = Vec::new();
    let mut index = 0;
    // TODO: traverse the list achieved by the selector
    for item in fragment.select(&selector) {
        // TODO: filter the text of the element with id shindanName
        if Some("shindanResult") == item.value().attr("id") {
            // TODO: traverse the list achieved by the text of the element with id shindanName
            for item in item.text().to_owned() {
                if index == 6 || index == 0 {
                    info_list.push(item.to_owned());
                } else {
                    let right = item.split(":").last();
                    if let Some(right) = right {
                        info_list.push(right.to_string())
                    }
                }
                index += 1;
            }
        }
    }
    let result = Datas {
        name,
        sex: &info_list[1],
        race: &info_list[2],
        charactor: &info_list[3],
        talent: &info_list[4],
        camp: &info_list[5],
        hobby: &info_list[6],
        hair: &info_list[7],
        pupil: &info_list[8],
        danger: &info_list[9],
        lucky: &info_list[10],
    };
    "String".to_string()
    Ok(result)
}

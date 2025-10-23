use futures::future::{self};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Icon {
    icon_name: String,
    icon_code: String,
}

async fn fetch_page(client: &Client, url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let res = client.get(url).send().await?;
    if !res.status().is_success() {
        return Err(format!("Failed to fetch URL: {} with status {}", url, res.status()).into());
    }
    let body = res.text().await?;
    Ok(body)
}

async fn fetch_icons() -> Result<Vec<Icon>, Box<dyn Error + Send + Sync>> {
    let base_url = "https://api.flutter.dev/flutter/cupertino/CupertinoIcons-class.html";

    let client = Client::new();

    let body = fetch_page(&client, base_url).await?;
    let document = Document::from(body.as_str());

    let tasks: Vec<_> = document
        .find(Attr("id", "constants").descendant(Attr("class", "constant")))
        .filter(|node| {
            let icon_name = node.find(Name("a")).next().unwrap().text();
            icon_name != "iconFont" && icon_name != "iconFontPackage"
        })
        .map(|node| {
            let client = client.clone();

            let icon_name = node.find(Name("a")).next().unwrap().text();
            let detail_href = node
                .find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap()
                .to_string();

            tokio::spawn(async move {
                let detail_url = format!("https://api.flutter.dev/flutter/{}", detail_href);

                match fetch_icon_code(&client, &detail_url).await {
                    Ok(icon_code) => Ok(Icon {
                        icon_name,
                        icon_code,
                    }),
                    Err(e) => Err(format!("Failed to load icon '{}': {}", icon_name, e)),
                }
            })
        })
        .collect();

    let results = future::join_all(tasks).await;

    let mut icons = Vec::new();
    for result in results {
        match result {
            Ok(Ok(icon)) => icons.push(icon),
            Ok(Err(e)) => eprintln!("{}", e),
            Err(e) => eprintln!("Task failed to execute: {}", e),
        }
    }

    if icons.is_empty() {
        return Err("No icons were found. The website structure may have changed again.".into());
    }

    Ok(icons)
}

async fn fetch_icon_code(
    client: &Client,
    url: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let body = fetch_page(client, url).await?; // 使用 .await
    let html = body.as_str();
    let document = Document::from(html);

    let number = document
        .find(Name("code").and(Class("language-dart")))
        .next();
    match number {
        Some(item) => {
            let text = item.text();
            let code = extract_hex_code(&text);
            match code {
                Some(code) => Ok(code),
                None => Err(format!("没有解析到 icon图标, {}", url).into()),
            }
        }
        None => Err(format!("在页面上找不到图标代码点: {}", url).into()),
    }
}
fn extract_hex_code(text: &str) -> Option<String> {
    static HEX_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"0x[[:xdigit:]]+").expect("无效的正则表达式"));
    HEX_REGEX.find(text).map(|mat| mat.as_str().to_string())
}
#[tokio::main]
async fn main() {
    match fetch_icons().await {
        Ok(icons) => match serde_json::to_string_pretty(&icons) {
            Ok(json_str) => match std::fs::write("icons.json", &json_str) {
                Ok(_) => println!(
                    "Successfully wrote icons to icons.json, size: {}",
                    icons.len()
                ),
                Err(e) => eprintln!("Error writing to file: {}", e),
            },
            Err(e) => eprintln!("Error serializing to JSON: {}", e),
        },
        Err(e) => {
            eprintln!("Error fetching icons: {}", e);
        }
    }
}

use futures::lock::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Client, Error};

pub type TranslateCache = Mutex<HashMap<String, String>>;

pub async fn translate(cache: &TranslateCache, tl: &str, q: &str) -> String {
    let cv = cache.lock().await.get(q).map(|e| e.clone());
    match cv {
        Some(x) => x.clone(),
        None => {
            let x = translate_request(tl, q).await.unwrap_or(String::new());
            cache.lock().await.insert(q.to_string(), x.clone());
            x
        }
    }
}

async fn translate_request(tl: &str, q: &str) -> Result<String, Error> {
    let resp = Client::builder()
        .user_agent("Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.5563.116 Mobile Safari/537.36")
        .timeout(std::time::Duration::new(5, 0))
        .build().unwrap()
        .get("https://translate.google.com/m?")
        .query(&[("sl", "en"), ("tl", tl), ("q", q)])
        .send().await?
        .text().await?;
    lazy_static! {
        static ref RE: Regex = Regex::new(r#""result-container">(.*?)</div>"#).unwrap();
    }
    let caps = RE.captures(resp.as_str());
    Ok(caps.and_then(|e|
        e.get(1).map(|e| e.as_str().to_string())).unwrap_or(String::new())
    )
}

use crate::reddit::{DataTrait, RedditChildrenWrapper, RedditResponse};
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client,
};
use std::option;

pub fn get_reddit_url(sub_reddit: &str) -> String {
    let url = format!("https://www.reddit.com/r/{}.json", sub_reddit);
    return url;
}

pub async fn request_reddit(url: &String) -> option::Option<Vec<RedditChildrenWrapper>> {
    let mut header = HeaderMap::new();
    header.append(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"));
    let response = Client::new()
        .get(url)
        .headers(header)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let response = serde_json::from_str::<RedditResponse>(response.as_str()).unwrap();

    let mut children = response.data.children;

    children.sort_by(|a, b| a.data.get_weight().cmp(&b.data.get_weight()));

    for child in &children[..3] {
        println!("=========================================");
        println!("{}", child.data);
        println!("=========================================");
        print!("\n\n\n")
    }

    None
}

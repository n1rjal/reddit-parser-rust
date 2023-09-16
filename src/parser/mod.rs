use crate::reddit::CliArgument;
use crate::reddit::{DataTrait, RedditChildrenWrapper, RedditResponse};
use clap::{App, Arg};
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client,
};

pub fn get_reddit_url(sub_reddit: &str) -> String {
    let url = format!("https://www.reddit.com/r/{}.json", sub_reddit);
    return url;
}

pub async fn request_reddit(url: &String) -> Vec<RedditChildrenWrapper> {
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

    let response = serde_json::from_str::<RedditResponse>(response.as_str())
        .expect("API DATA SCHEMA CHANGED, FAILED TO PARSE");

    let mut children = response.data.children;

    children.sort_by(|a, b| a.data.get_weight().cmp(&b.data.get_weight()));
    children
}

pub fn parse_command_line_arg() -> CliArgument {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Nirjal Paudel <nirjalpaudel54312@hack.gov>")
        .arg(
            Arg::with_name("sr")
                .short("sr")
                .long("sub-reddit")
                .takes_value(true)
                .help("Sub reddit you want to parse posts from "),
        )
        .arg(
            Arg::with_name("num")
                .short("n")
                .long("top-n")
                .takes_value(true)
                .help("Top n posts to parse"),
        )
        .get_matches();

    let sub_reddit = matches
        .value_of("sr")
        .expect("sub reddit name must be provided");

    CliArgument {
        sub_reddit: String::from(sub_reddit),
        top_n: matches
            .value_of("num")
            .expect("A numeric string is expected")
            .parse()
            .expect("Top n must be a valid integer number"),
    }
}

extern crate web_scrape;

#[tokio::main]
async fn main() {
    let subreddit_name = "technepal";
    let sub_reddit_url = web_scrape::parser::get_reddit_url(subreddit_name);
    let sub_reddit_data = web_scrape::parser::request_reddit(&sub_reddit_url).await;

    println!("{:?}", serde_json::to_string(&sub_reddit_data).unwrap());
}

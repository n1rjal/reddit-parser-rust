extern crate tokio;
extern crate web_scrape;

#[tokio::main]
async fn main() {
    let cli_args = web_scrape::parser::parse_command_line_arg();
    println!(
        "Parsing subreddit {} for top {}",
        cli_args.sub_reddit, cli_args.top_n
    );
    let sub_reddit_url = web_scrape::parser::get_reddit_url(&cli_args.sub_reddit);
    let sub_reddit_data = web_scrape::parser::request_reddit(&sub_reddit_url).await;
    web_scrape::display::display_subreddit_data(sub_reddit_data, cli_args.top_n);
}

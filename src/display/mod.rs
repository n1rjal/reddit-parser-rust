use crate::reddit::RedditChildrenWrapper;

pub fn display_subreddit_data(post_data: Vec<RedditChildrenWrapper>, top_n: usize) {
    print!("\n\n");
    for child in &post_data[..top_n] {
        println!("{}", child.data);
        print!("\n\n")
    }
}

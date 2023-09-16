extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum BoolOrNumber {
    Bool(bool),
    Number(usize),
}

#[derive(Serialize, Deserialize)]
pub struct RedditResponse {
    #[serde(rename = "kind")]
    pub kind: String,

    #[serde(rename = "data")]
    pub data: RedditResponseWrapper,
}

#[derive(Serialize, Deserialize)]
pub struct RedditResponseWrapper {
    #[serde(rename = "after")]
    after: String,

    #[serde(rename = "after")]
    before: Option<String>,

    #[serde(rename = "dist")]
    dist: i16,

    #[serde(rename = "modhash")]
    modhash: String,

    #[serde(rename = "children")]
    pub children: Vec<RedditChildrenWrapper>,
}

#[derive(Serialize, Deserialize)]
pub struct RedditChildrenWrapper {
    #[serde(rename = "kind")]
    kind: String,

    #[serde(rename = "data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "selftext")]
    #[serde(default)]
    self_text: Option<String>,

    #[serde(rename = "title")]
    #[serde(default)]
    title: String,

    #[serde(rename = "upvote_ratio")]
    #[serde(default)]
    upvote_ratio: f64,

    #[serde(rename = "ups")]
    #[serde(default)]
    ups: i64,

    #[serde(rename = "total_awards_received")]
    #[serde(default)]
    total_awards_received: i64,

    #[serde(rename = "likes")]
    #[serde(default)]
    likes: Option<serde_json::Value>,

    #[serde(rename = "author")]
    #[serde(default)]
    author: String,

    #[serde(rename = "url")]
    #[serde(default)]
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Gildings {}

#[derive(Serialize, Deserialize)]
pub struct LinkFlairRichtext {
    #[serde(rename = "e")]
    e: String,

    #[serde(rename = "t")]
    t: String,
}

#[derive(Serialize, Deserialize)]
pub struct Preview {
    #[serde(rename = "images")]
    images: Vec<Image>,

    #[serde(rename = "enabled")]
    enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "source")]
    source: Source,

    #[serde(rename = "resolutions")]
    resolutions: Vec<Source>,

    #[serde(rename = "variants")]
    variants: Gildings,

    #[serde(rename = "id")]
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "width")]
    width: i64,

    #[serde(rename = "height")]
    height: i64,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let self_text = match &self.self_text {
            Some(text) => text.as_str(),
            None => r"\c",
        };

        match self_text {
            r"\c" => write!(
                f,
                "{}\nAuthor:{}\nhttps://reddit.com{}",
                self.title, self.author, self.url
            ),
            _ => write!(
                f,
                "{}\nAuthor: {}\n{}\n\n{}",
                self.title, self.author, self.url, self_text
            ),
        }
    }
}

pub struct CliArgument {
    pub sub_reddit: String,
    pub top_n: usize,
}

pub trait DataTrait {
    fn get_weight(&self) -> i32;
}

impl DataTrait for Data {
    fn get_weight(&self) -> i32 {
        let mut weight = 0;
        weight += i32::from((self.upvote_ratio * f64::from(10000)) as i32);
        weight += self.total_awards_received as i32 * i32::from(2);
        weight
    }
}

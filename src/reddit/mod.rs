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

    #[serde(rename = "geo_filter")]
    geo_filter: Option<i32>,

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
    #[serde(rename = "approved_at_utc")]
    approved_at_utc: Option<serde_json::Value>,

    #[serde(rename = "subreddit")]
    subreddit: Option<String>,

    #[serde(rename = "selftext")]
    self_text: Option<String>,

    #[serde(rename = "author_fullname")]
    author_full_name: String,

    #[serde(rename = "saved")]
    saved: bool,

    #[serde(rename = "mod_reason_title")]
    mod_reason_title: Option<serde_json::Value>,

    #[serde(rename = "gilded")]
    gilded: i64,

    #[serde(rename = "clicked")]
    clicked: bool,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "link_flair_richtext")]
    link_flair_richtext: Vec<LinkFlairRichtext>,

    #[serde(rename = "subreddit_name_prefixed")]
    subreddit_name_prefixed: String,

    #[serde(rename = "hidden")]
    hidden: bool,

    #[serde(rename = "pwls")]
    pwls: i64,

    #[serde(rename = "link_flair_css_class")]
    link_flair_css_class: String,

    #[serde(rename = "downs")]
    downs: i64,

    #[serde(rename = "thumbnail_height")]
    thumbnail_height: Option<i64>,

    #[serde(rename = "top_awarded_type")]
    top_awarded_type: Option<serde_json::Value>,

    #[serde(rename = "hide_score")]
    hide_score: bool,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "quarantine")]
    quarantine: bool,

    #[serde(rename = "link_flair_text_color")]
    link_flair_text_color: String,

    #[serde(rename = "upvote_ratio")]
    upvote_ratio: f64,

    #[serde(rename = "author_flair_background_color")]
    author_flair_background_color: Option<serde_json::Value>,

    #[serde(rename = "ups")]
    ups: i64,

    #[serde(rename = "total_awards_received")]
    total_awards_received: i64,

    #[serde(rename = "media_embed")]
    media_embed: Gildings,

    #[serde(rename = "thumbnail_width")]
    thumbnail_width: Option<i64>,

    #[serde(rename = "author_flair_template_id")]
    author_flair_template_id: Option<serde_json::Value>,

    #[serde(rename = "is_original_content")]
    is_original_content: bool,

    #[serde(rename = "user_reports")]
    user_reports: Vec<Option<serde_json::Value>>,

    #[serde(rename = "secure_media")]
    secure_media: Option<serde_json::Value>,

    #[serde(rename = "is_reddit_media_domain")]
    is_reddit_media_domain: bool,

    #[serde(rename = "is_meta")]
    is_meta: bool,

    #[serde(rename = "category")]
    category: Option<serde_json::Value>,

    #[serde(rename = "secure_media_embed")]
    secure_media_embed: Gildings,

    #[serde(rename = "link_flair_text")]
    link_flair_text: String,

    #[serde(rename = "score")]
    score: i64,

    #[serde(rename = "approved_by")]
    approved_by: Option<serde_json::Value>,

    #[serde(rename = "is_created_from_ads_ui")]
    is_created_from_ads_ui: bool,

    #[serde(rename = "author_premium")]
    author_premium: bool,

    #[serde(rename = "thumbnail")]
    thumbnail: String,

    // #[serde(rename = "edited")]
    // edited: BoolOrNumber,
    #[serde(rename = "author_flair_css_class")]
    author_flair_css_class: Option<serde_json::Value>,

    #[serde(rename = "author_flair_richtext")]
    author_flair_richtext: Vec<Option<serde_json::Value>>,

    #[serde(rename = "gildings")]
    gildings: Gildings,

    #[serde(rename = "post_hint")]
    post_hint: Option<String>,

    #[serde(rename = "content_categories")]
    content_categories: Option<serde_json::Value>,

    #[serde(rename = "is_self")]
    is_self: bool,

    #[serde(rename = "subreddit_type")]
    subreddit_type: String,

    #[serde(rename = "created")]
    created: f64,

    #[serde(rename = "link_flair_type")]
    link_flair_type: String,

    #[serde(rename = "wls")]
    wls: f64,

    #[serde(rename = "removed_by_category")]
    removed_by_category: Option<serde_json::Value>,

    #[serde(rename = "banned_by")]
    banned_by: Option<serde_json::Value>,

    #[serde(rename = "author_flair_type")]
    author_flair_type: String,

    #[serde(rename = "domain")]
    domain: String,

    #[serde(rename = "selftext_html")]
    self_text_html: Option<serde_json::Value>,

    #[serde(rename = "likes")]
    likes: Option<serde_json::Value>,

    #[serde(rename = "suggested_sort")]
    suggested_sort: Option<serde_json::Value>,

    #[serde(rename = "banned_at_utc")]
    banned_at_utc: Option<serde_json::Value>,

    #[serde(rename = "url_overridden_by_dest")]
    url_overridden_by_dest: Option<String>,

    #[serde(rename = "view_count")]
    view_count: Option<serde_json::Value>,

    #[serde(rename = "preview")]
    preview: Option<Preview>,

    #[serde(rename = "all_awardings")]
    all_awarding: Vec<Option<serde_json::Value>>,

    #[serde(rename = "awarders")]
    awarders: Vec<Option<serde_json::Value>>,

    #[serde(rename = "link_flair_template_id")]
    link_flair_template_id: String,

    #[serde(rename = "author_flair_text")]
    author_flair_text: Option<serde_json::Value>,

    #[serde(rename = "treatment_tags")]
    treatment_tags: Vec<Option<serde_json::Value>>,

    #[serde(rename = "removed_by")]
    removed_by: Option<serde_json::Value>,

    #[serde(rename = "mod_note")]
    mod_note: Option<serde_json::Value>,

    #[serde(rename = "distinguished")]
    distinguished: Option<serde_json::Value>,

    #[serde(rename = "subreddit_id")]
    subreddit_id: String,

    #[serde(rename = "author_is_blocked")]
    author_is_blocked: bool,

    #[serde(rename = "mod_reason_by")]
    mod_reason_by: Option<serde_json::Value>,

    #[serde(rename = "num_reports")]
    num_reports: Option<serde_json::Value>,

    #[serde(rename = "removal_reason")]
    removal_reason: Option<serde_json::Value>,

    #[serde(rename = "link_flair_background_color")]
    link_flair_background_color: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "is_robot_indexable")]
    is_robot_indexable: bool,

    #[serde(rename = "report_reasons")]
    report_reasons: Option<serde_json::Value>,

    #[serde(rename = "author")]
    author: String,

    #[serde(rename = "discussion_type")]
    discussion_type: Option<serde_json::Value>,

    #[serde(rename = "num_comments")]
    num_comments: i64,

    #[serde(rename = "send_replies")]
    send_replies: bool,

    #[serde(rename = "whitelist_status")]
    whitelist_status: String,

    #[serde(rename = "contest_mode")]
    contest_mode: bool,

    #[serde(rename = "mod_reports")]
    mod_reports: Vec<Option<serde_json::Value>>,

    #[serde(rename = "author_patreon_flair")]
    author_patreon_flair: bool,

    #[serde(rename = "author_flair_text_color")]
    author_flair_text_color: Option<serde_json::Value>,

    #[serde(rename = "permalink")]
    permalink: String,

    #[serde(rename = "parent_whitelist_status")]
    parent_whitelist_status: String,

    #[serde(rename = "stickied")]
    stickied: bool,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "subreddit_subscribers")]
    subreddit_subscribers: i64,

    #[serde(rename = "created_utc")]
    created_utc: f64,

    #[serde(rename = "num_crossposts")]
    num_crossposts: usize,

    #[serde(rename = "media")]
    media: Option<serde_json::Value>,

    #[serde(rename = "is_video")]
    is_video: bool,
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
            r"\c" => write!(f, ""),
            _ => write!(f, "Author: {}\n{}", self.author, self_text),
        }
    }
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

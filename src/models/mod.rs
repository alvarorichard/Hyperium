use std::time::Duration;

pub const BASE_URL: &str = "https://vizertv.in/";
pub const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";
pub const ENCONDING: &str = "gzip, deflate, br, zstd";

pub mod movie;

#[derive(Debug)]
pub enum MediaType {
    TvShow,
    Movie,
    Anime,
}

pub struct Media {
    pub title: String,
    pub url: String,
    pub year: String,
    pub duration: Option<Duration>,
    pub media_type: MediaType,
    pub description: String,
}

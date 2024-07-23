use std::time::Duration;
pub mod common;
pub mod movie;

#[derive(Debug)]
pub struct Media {
    pub title: String,
    pub pictures: Vec<Pic>,
    pub url: String,
    pub year: String,
    pub duration: Option<Duration>,
    pub media_type: MediaType,
    pub description: String,
}

#[derive(Debug)]
pub enum MediaType {
    TvShow,
    Movie,
    Anime,
    Unknown,
}
#[derive(Debug)]
pub enum PicFormat {
    Webp,
    Jpeg,
    Png,
}
#[derive(Debug)]
pub struct Pic {
    pub url: String,
    pub format: PicFormat,
}

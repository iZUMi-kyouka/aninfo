use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use yew::prelude::*;

#[derive(Store, Default, PartialEq, Debug, Clone)]
pub struct NavbarSearch {
    pub query: String
}

#[derive(Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct PaginationObj {
    pub last_visible_page: i32,
    pub has_next_page: bool,
    pub current_page: u8,
    pub items: PaginationItems
}

#[derive(Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct PaginationItems {
    pub count: i32,
    pub total: i32,
    pub per_page: i32
}

#[derive(Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct Images {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>
}

#[derive(Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct AnimeImages {
    pub jpg: Images,
    pub webp: Images
}

#[derive(Store, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct Trailers {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>
}

#[derive(Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct Title {
    pub r#type: String,
    pub title: String
}

impl Title {
    pub fn get_type(&self) -> String {
        (&self.r#type).to_string()
    }
}


#[derive(Store, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct TestObj {
    p1: u8,
    p2: u8,
    p3: u8,
    p4: u8
}

#[derive(Properties, Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct AnimeObj {
    pub mal_id: u64,
    pub url: String,
    pub images: AnimeImages,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub source: Option<String>,
    pub episodes: Option<u16>,
    pub status: Option<String>,
    pub airing: bool,
    pub score: Option<f32>,
    pub scored_by: Option<u64>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub year: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub studios: Vec<MALObj>,
    pub genres: Vec<MALObj>,
    pub season: Option<String>
}


#[derive(Properties, Store, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct QueryResult {
    pub data: Vec<AnimeObj>,
    pub pagination: PaginationObj
}

#[derive(Properties, Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct AnimeObjAsProp {
    pub anime_obj: AnimeObj
}

#[derive(Properties, Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct MALObj {
    pub mal_id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String
}

#[derive(Properties, Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct CharObj {
    pub mal_id: u32,
    pub url: String,
    pub images: CharImgWrapper,
    pub name: String,

}

#[derive(Store, PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
pub struct Char {
    pub character: CharObj,
    pub role: String
}

#[derive(Store, PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
pub struct CharWrapper {
    pub data: Vec<Char>
}

#[derive(Store, PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
pub struct CharImgWrapper {
    pub jpg: CharImg,
    pub webp: CharImg
}

#[derive(Store, PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
pub struct CharImg {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AnimeEpisode {
    pub mal_id: u32,
    pub url: Option<String>,
    pub title: String,
    pub title_japanese: Option<String>,
    pub title_romanji: Option<String>,
    pub aired: Option<String>,
    pub forum_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AnimeEpisodeWrapper {
    pub data: Vec<AnimeEpisode>,
    pub pagination: AnimeEpisodePagination
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AnimeEpisodePagination {
    pub last_visible_page: u8,
    pub has_next_page: bool
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SeasonObj {
    pub year: u32,
    pub seasons: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SeasonsWrapper {
    pub pagination: AnimeEpisodePagination,
    pub data: Vec<SeasonObj>
}
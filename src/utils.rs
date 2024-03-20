use std::iter::repeat_with;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use yew_router::navigator::Navigator;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux::Dispatch;

use crate::components::stores::AnimeObj;
use crate::components::{appcontext_provider::{AppContext, AppCtx, Theme}, stores::NavbarSearch};
use crate::components::anime_card::AnimeCard;

pub fn handle_theme(app_ctx: &AppContext) -> &'static str {
    if let &Theme::Light(class) = &((*app_ctx).theme) {
        class
    } else if let &Theme::Dark(class) = &((*app_ctx).theme) {
        class
    } else {
        ""
    }
}

pub fn handle_nsfw(app_ctx: &AppContext) -> &'static str {
    if (*app_ctx).nsfw {
        "&sfw=false"
    } else {
        "&sfw=true"
    }
}

pub async fn fetch_data_into<T: Serialize + DeserializeOwned> (link: &str) -> T {
    reqwasm::http::Request::get(link)
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap()
}

pub fn handle_season(month: &str) -> &'static str {
    let month = month.parse::<u8>().unwrap();
    if month == 1 || month == 2 || month == 3 {
        "Winter"
    } else if month == 4 || month == 5 || month == 6 {
        "Spring"
    } else if month == 7 || month == 8 || month == 9 {
        "Summer"
    } else if month == 10 || month == 11 || month == 12 {
        "Fall"
    } else {
        panic!("Invalid season.")
    }
}

pub fn into_char_cards(v: Vec<AnimeObj>) -> Html {
    v.into_iter().map(|anime_obj| {
        html! {
            <AnimeCard anime_obj={anime_obj}/>
        }
    }).collect::<Html>()
}
use yew::prelude::*;
use yew_router::prelude::*;

use super::{about_page::AboutPage, anime_desc::AnimeDesc, anime_result::AnimeResult, anime_result_v2::AnimeResultV2, home_page::HomePage, loading::Loading, privacy_policy::PrivacyPolicy};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/anime_result/:page/:query/:content_title")]
    AnimeResult {page: u8, query: String, content_title: String},
    #[at("/anime_result_v2")]
    AnimeResultV2,
    #[at("/anime_description")]
    AnimeDescription,
    #[at("/loading")]
    Loading,
    #[at("/privacy_policy")]
    PrivacyPolicy
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{<HomePage/>},
        Route::About => html!{<AboutPage/>},
        Route::AnimeResult {page, query, content_title} => html!{<AnimeResult page={page} query={query} content_title={content_title}/>},
        Route::AnimeDescription => html!{<AnimeDesc/>},
        Route::Loading => html!{<Loading/>},
        Route::PrivacyPolicy => html!{<PrivacyPolicy/>},
        Route::AnimeResultV2 => html!{<AnimeResultV2/>}
    }
}
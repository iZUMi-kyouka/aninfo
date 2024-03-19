use yew::prelude::*;
use yew_router::prelude::*;

use super::{about_page::AboutPage, anime_desc::AnimeDesc, anime_result::AnimeResult, anime_result_v2::AnimeResultV2, home_page::HomePage, loading::Loading, privacy_policy::PrivacyPolicy};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/anime_result/:page")]
    AnimeResult {page: u8},
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
        Route::AnimeResult {page} => html!{<AnimeResult page={page}/>},
        Route::AnimeDescription => html!{<AnimeDesc/>},
        Route::Loading => html!{<Loading/>},
        Route::PrivacyPolicy => html!{<PrivacyPolicy/>},
        Route::AnimeResultV2 => html!{<AnimeResultV2/>}
    }
}
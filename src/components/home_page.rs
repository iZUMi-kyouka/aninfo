use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::components::stores::SeasonsWrapper;
use crate::components::{appcontext_provider::AppContext, stores::NavbarSearch};
use crate::components::routes::Route;
use crate::utils::{fetch_data_into, handle_season, handle_theme, into_char_cards};
use crate::components::stores::*;

#[function_component(HomePage)]
pub fn name() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();
    let (nbs_state, nbs_disp) = use_store::<NavbarSearch>();
    let cur_season = use_state(|| "".to_string());
    let year = use_state(|| "".to_string());
    let top: UseStateHandle<Vec<AnimeObj>> = use_state(|| Vec::new());
    let seasonal: UseStateHandle<Vec<AnimeObj>> = use_state(|| Vec::new());

    {
        let app_ctx = app_ctx.clone();
        let nav = nav.clone();
        let (nbs_state, nbs_disp) = (nbs_state.clone(), nbs_disp.clone());
        let cur_season = cur_season.clone();
        let year = year.clone();
        let top = top.clone();
        let seasonal = seasonal.clone();

        use_effect_with((*app_ctx).clone(), move |_| {
            let seasonal = seasonal.clone();
            let top = top.clone();
            web_sys::window().unwrap().document().unwrap().set_title(&format!("ANiNFO: {}", "Home"));
            let cur_season = cur_season.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let month = chrono::Local::now().to_string();
                year.set(month[..4].to_string());
                cur_season.set(handle_season(&month[5..7]).to_string());

                //TOP ANIME
                let mut temp_vec: Vec<AnimeObj> = Vec::new();
                
                for i in (1..2) {
                    let data = fetch_data_into::<QueryResult>(&format!("https://api.jikan.moe/v4/anime?order_by=score&start_date=2023-{}-01&end_date=2024-{}-01&sort=desc&type=tv&page={}", {
                        let mut cur_month = month[5..7].parse::<u8>().unwrap();
                        if cur_month <= 6 {
                            cur_month = (cur_month)+12-6
                        } else {
                            cur_month = cur_month - 6
                        }

                        if cur_month < 10 {
                            format!("0{}", cur_month)
                        } else {
                            format!("{}", cur_month)
                        }
                    },&month[5..7], i)).await;
                    temp_vec.append(&mut data.data.into_iter().collect::<Vec<AnimeObj>>());
                }

                seasonal.set(temp_vec.clone());
                temp_vec.clear();

                for i in (1..2) {
                    let data = fetch_data_into::<QueryResult>(&format!("https://api.jikan.moe/v4/top/anime?page={}", i)).await;
                    temp_vec.append(&mut data.data.into_iter().collect::<Vec<AnimeObj>>());

                top.set(temp_vec.clone());
                temp_vec.clear();
            }})
        });
    }

    html!(
        <div>
        <h1 class="content-heading">{format!("{} {}'s Selection", ((*cur_season).clone()), ((*year).clone()))}</h1>
        <div class={format!("home-page-wrapper {}", handle_theme(&app_ctx))}>

        <div class="home-seasonal">
        {into_char_cards((*seasonal).clone())}
        </div>
        </div><br/><br/>
        <h1 class="content-heading">{"Anime of All Time"}</h1>
        <div class="home-page-wrapper">
        <div class="home-top">
        {into_char_cards((*top).clone())}
        </div>
        </div>
        </div>
    )
}
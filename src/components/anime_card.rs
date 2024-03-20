use yew::prelude::*;
use yew_router::prelude::*;
use crate::{components::{animeobjcontext_provider::AnimeObjContext, appcontext_provider::{AppContext, Language}}, utils::handle_theme, Route};

use super::stores::{AnimeObj, AnimeObjAsProp};

#[function_component(AnimeCard)]
pub fn anime_card(anime_obj: &AnimeObjAsProp) -> Html {
    let theme = handle_theme(&use_context::<AppContext>().unwrap());
    let ao_ctx = use_context::<AnimeObjContext>().unwrap();
    let app_ctx = use_context::<AppContext>().unwrap();

    let nav = use_navigator().unwrap();

    let go_to_desc = {
        let anime_obj = (anime_obj).clone();
        let nav = nav.clone();
        let ao_ctx = ao_ctx.clone();
        let app_ctx = app_ctx.clone();
        Callback::from(move |event: MouseEvent| {
            ao_ctx.dispatch(anime_obj.anime_obj.clone());
            app_ctx.dispatch((*app_ctx).update_loading_page_into(true));
            nav.push(&Route::AnimeDescription);
        })
    };

    let mut en_ttl = None;
    let mut jp_ttl = None;

    let titles = &(anime_obj.anime_obj.titles);
    for title in titles {
        if title.get_type() == "English".to_string() {
            en_ttl = Some((&title.title).clone())
        } else if title.get_type() == "Japanese".to_string() {
            jp_ttl = Some((&title.title).clone())
        }
    }

    let mut title_rendered = "".to_string();

    match &((*app_ctx).language) {
        &Language::EN => {
            if let Some(t) = en_ttl {
                title_rendered = t
            } else {
                title_rendered = (anime_obj.anime_obj.titles[0].title).to_string();
            }
        },

        &Language::JP => {
            if let Some(t) = jp_ttl {
                title_rendered = t;
            } else {
                title_rendered = (anime_obj.anime_obj.titles[0].title).to_string();
            }
        }
    }

    let title_rendered = title_rendered.to_string();
    
    html! {
        <div class={format!("ani-card {}", theme)}>
            <div class="card-desc">
            <a onclick={(&go_to_desc).clone()} style={"cursor: pointer"}><h4 id="ani-title">{format!("{}", title_rendered)}</h4></a>
            // <div id="ani-yr">{format!("{}", &(anime_obj.anime_obj.year.as_ref().clone().unwrap_or(&0)))}</div>
            // <div id="ani-yr"><i>{format!("{}", &(anime_obj.anime_obj.year.as_ref().unwrap_or(&0)))}</i></div>
            <div id="ani-rating">{format!("Rating: {} / 10.0", &(anime_obj.anime_obj.score.as_ref().unwrap_or(&0.0)))}</div>
            <div id="ani-stat">{format!("Status: {}", &(anime_obj.anime_obj.status.as_ref().unwrap_or(&"Unavailable".to_string())))}</div>
            </div>
            <div class="card-img">
            <a onclick={go_to_desc} style={"cursor: pointer"}>
            <img src={(anime_obj.anime_obj.images.jpg.image_url.as_ref().unwrap().clone())}/>
            </a>
            </div>
        </div>
    }
}
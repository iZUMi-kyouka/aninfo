use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{components::{animeobjcontext_provider::AnimeObjContext, appcontext_provider::{AppContext, Language}, char_card::CharCard}, utils::handle_theme, Route};

use super::stores::{AnimeObj, AnimeObjAsProp, CharWrapper};

#[function_component(AnimeDesc)]
pub fn anime_desc() -> Html {
    let theme = handle_theme(&use_context::<AppContext>().unwrap());
    let anime_obj = use_context::<AnimeObjContext>().unwrap();
    let app_ctx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();
    let (char_s, char_disp) = use_store::<CharWrapper>();
    
    if let None = anime_obj.anime_obj.as_ref() {
        nav.replace(&Route::Home);
    }

    let mut en_ttl = None;
    let mut jp_ttl = None;

    let titles = &(anime_obj.anime_obj.as_ref().unwrap().titles);
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
                title_rendered = (anime_obj.anime_obj.as_ref().unwrap().titles[0].title).to_string();
            }
        },

        &Language::JP => {
            if let Some(t) = jp_ttl {
                title_rendered = t;
            } else {
                title_rendered = (anime_obj.anime_obj.as_ref().unwrap().titles[0].title).to_string();
            }
        }
    }

    {
        let anime_obj = anime_obj.clone();
        let char_s = char_s.clone();
        let char_disp = char_disp.clone();
        use_effect_with(char_s, move |_| {
            let char_disp = char_disp.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = reqwasm::http::Request::get(&format!("https://api.jikan.moe/v4/anime/{}/characters", anime_obj.anime_obj.as_ref().unwrap().mal_id))
                .send()
                .await
                .unwrap()
                .json::<CharWrapper>()
                .await
                .unwrap();
            
                char_disp.set(result);
            });
        })
    }

    html! {
        <div class={format!("ani-desc {}", theme)}>
            <div class="header">
                
                <img src={(anime_obj.anime_obj.as_ref().unwrap().images.jpg.image_url.as_ref().unwrap().clone())}/>
                <div class="header-desc">
                <h3 id="ani-title">{format!("{}", title_rendered)}</h3>
                // <div id="ani-yr">{format!("{}", &(anime_obj.anime_obj.year.as_ref().clone().unwrap_or(&0)))}</div>
                <div class="ani-studios">{"Studios "}<b>{
                    (anime_obj
                        .anime_obj
                        .as_ref()
                        .unwrap()
                        .clone()
                        .studios
                        .into_iter()
                        .map(|studio_obj| {studio_obj.name})
                        .collect::<Vec<String>>()
                        .join(", ")
                )
                }</b>

                <div class="ani-genres">{"Genres "}<b>{
                    (anime_obj
                        .anime_obj
                        .as_ref()
                        .unwrap()
                        .clone()
                        .genres
                        .into_iter()
                        .map(|genre_obj| {genre_obj.name})
                        .collect::<Vec<String>>()
                        .join(" | ")
                )
                }</b>
                </div>

                <div id="ani-rating">{"Rating "}<b>{(anime_obj.anime_obj.as_ref().unwrap().score.as_ref().unwrap_or(&0.0))}</b>{" / 10.0"}</div>
                <div id="ani-eps">{"Episodes "}<b>{(anime_obj.anime_obj.as_ref().unwrap().episodes.as_ref().unwrap_or(&0))}</b></div>
                <div id="ani-stat">{"Status "}<b>{(anime_obj.anime_obj.as_ref().clone().unwrap().status.as_ref().unwrap_or(&"Unavailable".to_string()))}</b></div>
                <div id="ani-desc-p">
                    <p>{format!("{}", &(anime_obj.anime_obj.as_ref().unwrap().synopsis.as_ref().unwrap_or(&"Synopsis unavailable.".to_string())))}</p>
                </div>
                </div>
            </div>

            </div>
            
            <h2>{"Characters"}</h2>
            <div class="char-info">
            <div class="char-wrapper">
            {char_s.data.clone().into_iter().map(|char_role| {
                html! {
                    <CharCard char_obj = {char_role}/>
                }
            }).collect::<Html>()}
            </div>
            </div>
        </div>
    }
}
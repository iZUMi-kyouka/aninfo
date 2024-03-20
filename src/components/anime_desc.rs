use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{components::{animeobjcontext_provider::AnimeObjContext, appcontext_provider::{AppContext, Language, DARK_THEME}, char_card::CharCard, episode_card::EpisodeCard, stores::{AnimeEpisode, AnimeEpisodeWrapper}}, utils::handle_theme, Route};

use super::stores::{AnimeObj, AnimeObjAsProp, CharWrapper};

#[function_component(AnimeDesc)]
pub fn anime_desc() -> Html {
    let theme = handle_theme(&use_context::<AppContext>().unwrap());
    let anime_obj = use_context::<AnimeObjContext>().unwrap();
    let app_ctx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();
    let (char_s, char_disp) = use_store::<CharWrapper>();
    let anime_eps_vec: UseStateHandle<Vec<AnimeEpisode>> = use_state(|| Vec::new());
    
    if let None = anime_obj.anime_obj.as_ref() {
        nav.replace(&Route::Home);
        return html!()  
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
    let mut default_title = "".to_string();

    match &((*app_ctx).language) {
        &Language::EN => {
            if let Some(t) = en_ttl.clone() {
                title_rendered = t
            } else {
                title_rendered = (anime_obj.anime_obj.as_ref().unwrap().titles[0].title).to_string();
            }
        },

        &Language::JP => {
            if let Some(t) = jp_ttl.clone() {
                title_rendered = t;
            } else {
                title_rendered = (anime_obj.anime_obj.as_ref().unwrap().titles[0].title).to_string();
            }
        }
    }

    default_title = (anime_obj.anime_obj.as_ref().unwrap().titles[0].title).to_string();
    if let Some(t) = en_ttl.clone() {
        default_title = t.clone();
    }
    

    {
        let app_ctx = app_ctx.clone();
        let anime_obj = anime_obj.clone();
        let char_s = char_s.clone();
        let char_disp = char_disp.clone();
        let nav = nav.clone();
        let title_rendered = title_rendered.clone();
        let anime_eps_vec = anime_eps_vec.clone();

        use_effect_with(char_s, move |_| {
            let anime_eps_vec = anime_eps_vec.clone();
            web_sys::window().unwrap().document().unwrap().set_title(&format!("ANiNFO: {}", title_rendered));
            let nav_cloned = nav.clone();

            if (*app_ctx).loading_page == true {
                nav.push(&Route::Loading)
            }

            let char_disp = char_disp.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = reqwasm::http::Request::get(&format!("https://api.jikan.moe/v4/anime/{}/characters", anime_obj.anime_obj.as_ref().unwrap().mal_id))
                .send()
                .await
                .unwrap()
                .json::<CharWrapper>()
                .await
                .unwrap();
                
                let eps_data = reqwasm::http::Request::get(&format!("https://api.jikan.moe/v4/anime/{}/episodes", anime_obj.anime_obj.as_ref().unwrap().mal_id))
                    .send()
                    .await
                    .unwrap()
                    .json::<AnimeEpisodeWrapper>()
                    .await
                    .unwrap();

                let mut pagination_data = (eps_data.pagination).clone();
                let mut cur_page = 1u8;
                let mut temp_data: Vec<AnimeEpisode> = Vec::new();

                eps_data.data.iter().for_each({|eps_obj| {
                    temp_data.push(eps_obj.clone());
                }});

                while pagination_data.has_next_page {
                    let eps_data = reqwasm::http::Request::get(&format!("https://api.jikan.moe/v4/anime/{}/episodes?page={}", anime_obj.anime_obj.as_ref().unwrap().mal_id, cur_page+1))
                    .send()
                    .await
                    .unwrap()
                    .json::<AnimeEpisodeWrapper>()
                    .await
                    .unwrap();

                    pagination_data = (eps_data.pagination).clone();
                    eps_data.data.iter().for_each({|eps_obj| {
                        temp_data.push(eps_obj.clone());
                    }});
                    cur_page += 1;
                }

                log!(format!("{:?}", &temp_data));
                anime_eps_vec.set(temp_data);
            
                char_disp.set(result);
                if (*app_ctx).loading_page == true {
                    app_ctx.dispatch((*app_ctx).update_loading_page_into(false));
                    nav_cloned.back();
                    nav_cloned.replace(&Route::AnimeDescription)
                }
            });
        })
    }

    html! {
        <div class={format!("ani-desc {}", theme)}>
            <div class="header">
                <div id="header-img">
                <img src={(anime_obj.anime_obj.as_ref().unwrap().images.jpg.image_url.as_ref().unwrap().clone())}/>
                </div>
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

            <h2>{"Stream / Download"}</h2>
            <div class={format!("stream-wrapper {}", handle_theme(&app_ctx))}>
            <a target="_blank" href={format!("https://aniwave.to/filter?keyword={}", &default_title)}>
            <div class={format!("stream-card {}-emp", handle_theme(&app_ctx))}>

                    <img class={format!("stream-logo {}", theme)} src="static/aw.png"/>
                    <p>{"Watch on "}<b>{"Aniwave"}</b></p>
    
                </div>
                </a>
                <a target="_blank" href={format!("https://nyaa.si/?f=0&c=1_0&q={}&s=seeders&o=desc", &default_title)}>
                <div class={format!("stream-card {}-emp", handle_theme(&app_ctx))}>

                    <img class={format!("stream-logo {}", theme)} src="static/nyaa.png"/>
                    <p>{"Find on "}<b>{"Nyaa.si"}</b></p>
                </div>
                </a>
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

            <h2>{"Episodes"}</h2>
            <div class="eps-card-wrapper">
                {(*anime_eps_vec).clone().into_iter().map(|eps_obj| {
                    html!(<EpisodeCard anime_eps={eps_obj}/>)
                }).collect::<Html>()}
            </div>
        </div>
    }
}
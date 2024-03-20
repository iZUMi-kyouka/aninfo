use std::{rc::Rc, result};

use gloo::console::log;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;

use crate::{components::stores::AnimeEpisode, utils::{handle_nsfw, handle_theme}, Route};

use super::{appcontext_provider::AppContext, stores::{NavbarSearch, QueryResult}};
use crate::components::anime_card::AnimeCard;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub page: u8,
    pub query: String,
    pub content_title: String
}

#[function_component(AnimeResult)]
pub fn anime_result(props: &Props) -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().expect("Navigation unavailable error.");
    let (r_state, r_disp) = use_store::<QueryResult>();
    let (nbs_state, nbs_disp) = use_store::<NavbarSearch>();
    let theme = handle_theme(&app_ctx);
    let anime_episode = use_state(|| AnimeEpisode{..Default::default()});

    let query_prop = props.query.clone();
    let content_title = props.content_title.clone();

    {//use_effect_callback
        let (query_prop, content_title) = (query_prop.clone(), content_title.clone());
        let app_ctx = app_ctx.clone();
        let nbs_state = nbs_state.clone();
        let r_disp = r_disp.clone();
        let nav = nav.clone();

        use_effect_with(app_ctx.clone(), move |_| {
            // Handle title
            let app_ctx_cloned = app_ctx.clone();
            {
                if &(query_prop) == "seasonal" {
                    web_sys::window().unwrap().document().unwrap().set_title("ANiNFO | Seasonal Anime");
                }else {
                    let nbs_state = nbs_state.clone();
                    web_sys::window().unwrap().document().unwrap().set_title(&format!("ANiNFO | Search result for: {}", &(*nbs_state).query));
                }

            }

            wasm_bindgen_futures::spawn_local(async move {

                if (*app_ctx_cloned).loading_page == true {
                    let nav_used = nav.clone();
                    log!("Going to loading page...");
                    nav_used.push(&Route::Loading);
                }

                if &query_prop == "search" {
                    let result = reqwasm::http::Request::get(&format!("https://api.jikan.moe/v4/anime?q={}&page={}{}", nbs_state.query, (*app_ctx_cloned).cur_page, handle_nsfw(&app_ctx)))
                    .send()
                    .await
                    .unwrap()
                    .json::<QueryResult>()
                    .await
                    .unwrap();
    
                    r_disp.set(result);

                } else {

                    let result = reqwasm::http::Request::get(&format!("https://api.jikan.moe/v4/seasons/now?page={}{}", (*app_ctx_cloned).cur_page, handle_nsfw(&app_ctx)))
                    .send()
                    .await
                    .unwrap()
                    .json::<QueryResult>()
                    .await
                    .unwrap();

                    r_disp.set(result);
                }

                // log!((*app_ctx_cloned).loading_page);
                if (*app_ctx).loading_page.clone() {
                    let nav_used = nav.clone();
                    app_ctx.dispatch((*app_ctx).update_loading_page_into(false));
                    nav_used.replace(&Route::AnimeResult { page: 0 , query: (&query_prop).to_string(), content_title: (&content_title).to_string()})
                }
            });

            // nav.back();
            let win = web_sys::window().unwrap();
            win.scroll_to_with_x_and_y(0f64, 0f64);
            || {}
        })
    }
    
    html!{
        <>
        <h1 class={format!("content-heading {}", &theme)}>{content_title.clone()}</h1>
        <div class="page-btn-wrapper">
        <div class="page-btn">
        {(1..(*r_state).pagination.last_visible_page).map(|n| {
            // let goto_page = Rc::clone(&goto_page);
            // html!(<button>{n}</button>)
            if (*app_ctx).cur_page == n as u8 {
                html!(
                    <button class="btn-selected" onclick={
                        let app_ctx = app_ctx.clone();
                        let nav = nav.clone();
                        let (query_prop, content_title) = (query_prop.clone(), content_title.clone());
                        move |e: MouseEvent| {
                            app_ctx.dispatch((*app_ctx).update_page_into(n as u8).update_loading_page_into(true));
                            nav.push(&Route::AnimeResult { page: 0 , query: (&query_prop).to_string(), content_title: (&content_title).to_string()});
                        }
                    }><b>{n}</b></button>
                )
            } else {
                html!(<button onclick={
                    let app_ctx = app_ctx.clone();
                    let nav = nav.clone(); 
                    let (query_prop, content_title) = (query_prop.clone(), content_title.clone());
                    move |e: MouseEvent| {app_ctx.dispatch((*app_ctx).update_page_into(n as u8).update_loading_page_into(true)); 
                        nav.push(&Route::AnimeResult { page: 0 , query: {&query_prop}.to_string(), content_title: (&content_title).to_string()})}}>{n}</button>)
            }
            
        }).collect::<Html>()}
        </div>
        </div>
        <div class={format!("ani-result {}", &theme)}>
            // <h3>{"Debug Log"}</h3>
            // {format!("{:?}", *r_state)}
            {(*r_state).data.clone().into_iter().map(|anime_obj| {
                html! {
                    <AnimeCard anime_obj={anime_obj}/>
                }
            }).collect::<Html>()}
        </div>
        <div class={format!("ani-result-pages {}", &theme)}>
        //     {"Current Page: "}{&((*r_state).pagination.current_page)}
        // if (*r_state).pagination.has_next_page {
        //     <a onclick={next_page}>{"Next Page"}</a>
        // }

        // if (*r_state).pagination.current_page > 1 {
        //     <a onclick={prev_page}>{"Prev Page"}</a>
        // }
        <div class="page-btn-wrapper">
        <div class="fader"></div>
        <div class="page-btn fade-left fade-right">
        {(1..(*r_state).pagination.last_visible_page).map(|n| {
            // let goto_page = Rc::clone(&goto_page);
            // html!(<button>{n}</button>)
            if (*app_ctx).cur_page == n as u8 {
                html!(
                    <button class="btn-selected" onclick={
                        let app_ctx = app_ctx.clone();
                        move |e: MouseEvent| {
                            app_ctx.dispatch((*app_ctx).update_page_into(n as u8).update_loading_page_into(true))
                        }
                    }><b>{n}</b></button>
                )
            } else {
                html!(<button onclick={
                    let app_ctx = app_ctx.clone();
                    let nav = nav.clone(); 
                    move |e: MouseEvent| {app_ctx.dispatch((*app_ctx).update_page_into(n as u8).update_loading_page_into(true))}}>{n}</button>)
            }
            
        }).collect::<Html>()}
        </div>
        </div>
        </div>
        </>
    }
}
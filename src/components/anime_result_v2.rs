use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{components::{anime_card::AnimeCard, appcontext_provider::AppContext, stores::NavbarSearch}, utils::handle_theme, Route};

use super::stores::QueryResult;

#[derive(Properties, PartialEq)]
pub struct Props {
    page: u8
}

#[function_component(AnimeResultV2)]
pub fn anime_result_v2() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();
    let (nbs_state, _nbs_disp) = use_store::<NavbarSearch>();
    let (r_state, r_disp) = use_store::<QueryResult>();
    let nav = use_navigator().unwrap();

    {
        let app_ctx = app_ctx.clone();
        let nbs_state = nbs_state.clone();
        let r_disp = r_disp.clone();
        let nav = nav.clone();

        use_effect_with(app_ctx.clone(), move |_| {
            // let nav_used = nav.clone();
            // nav_used.push(&Route::Loading);
            wasm_bindgen_futures::spawn_local(async move {
                let result = reqwasm::http::Request::get(&format!("https://api.jikan.moe/v4/anime?q={}&page={}", nbs_state.query, (*app_ctx).cur_page))
                .send()
                .await
                .unwrap()
                .json::<QueryResult>()
                .await
                .unwrap();

            r_disp.set(result);
            });

            // nav.back();
            let win = web_sys::window().unwrap();
            win.scroll_to_with_x_and_y(0f64, 0f64);
            || {}
        })
    }

    let theme = handle_theme(&app_ctx);


    html!(
        <div>
        <h1 class={format!("ani-result-h3 {}", &theme)}>{"Search Result"}</h1>
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
            // {"Current Page: "}{&((*r_state).pagination.current_page)}
        // if (*r_state).pagination.has_next_page {
        //     <a onclick={next_page}>{"Next Page"}</a>
        // }

        // if (*r_state).pagination.current_page > 1 {
        //     <a onclick={prev_page}>{"Prev Page"}</a>
        // }

        // {(1..(*r_state).pagination.last_visible_page).map(|n| {
        //     let goto_page = Rc::clone(&goto_page);
        //     // html!(<button>{n}</button>)
        //     html!(<button onclick={page.set(n); (*goto_page).clone()}>{n}</button>)
        // }).collect::<Html>()}
        </div>
        <div class="page-btn">
        {(1..(*r_state).pagination.last_visible_page).map(|n| {
            // let goto_page = Rc::clone(&goto_page);
            // html!(<button>{n}</button>)
            if (*app_ctx).cur_page == n as u8 {
                html!(
                    <button class="btn-selected" onclick={
                        let app_ctx = app_ctx.clone();
                        move |e: MouseEvent| {
                            app_ctx.dispatch((*app_ctx).update_page_into(n as u8))
                        }
                    }><b>{n}</b></button>
                )
            } else {
                html!(<button onclick={
                    let app_ctx = app_ctx.clone();
                    let nav = nav.clone(); 
                    move |e: MouseEvent| {app_ctx.dispatch((*app_ctx).update_page_into(n as u8))}}>{n}</button>)
            }
            
        }).collect::<Html>()}
        </div>
        </div>
    )
}
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;
use crate::{components::{appcontext_provider::{AppContext, Language, Theme, AUTO_THEME, DARK_THEME, LIGHT_THEME}, stores::NavbarSearch}, Route};
use web_sys::{HtmlElement, HtmlInputElement};
use gloo::{console::log};
use wasm_bindgen::JsCast;
use yew::html_nested;

use super::stores::QueryResult;


#[function_component(TopNavbar)]
pub fn top_navbar() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let (q_state, q_disp) = use_store::<NavbarSearch>();
    let (r_state, r_disp) = use_store::<QueryResult>();

    let theme_toggle_onchange = {
        let app_ctx = app_ctx.clone();
        Callback::from(move |event: MouseEvent| {
            let cur_ctx = (*app_ctx).clone();
            if cur_ctx.theme == LIGHT_THEME {
                app_ctx.dispatch(app_ctx.update_theme_into(DARK_THEME))
            } else {
                app_ctx.dispatch(app_ctx.update_theme_into(LIGHT_THEME))
            }
        })
    };

    let language_toggle_onchange = {
        let app_ctx = app_ctx.clone();
        let nav_used = navigator.clone();
        let nav = navigator.clone();

        Callback::from(move |event: MouseEvent| {
            // nav_used.push(&Route::Loading);
            let cur_ctx = (*app_ctx).clone();
            if cur_ctx.language == Language::EN {
                app_ctx.dispatch(app_ctx.update_language_into(Language::JP))
            } else {
                app_ctx.dispatch(app_ctx.update_language_into(Language::EN))
            }

            // nav.back();
        })
    };

    let cur_mode = &((*app_ctx).theme);
    let mut theme_tog_label = "".to_string();
    match cur_mode {
        Theme::Light(_) => theme_tog_label = "🌙".to_string(),
        Theme::Dark(_) => theme_tog_label = "☀️".to_string(),
        _ => theme_tog_label = "AUTO".to_string()
    }

    let class_added = {
        if let &Theme::Light(class) = &((*app_ctx).theme) {
            class
        } else if let &Theme::Dark(class) = &((*app_ctx).theme) {
            class
        } else {
            ""
        }
    };

    let go_home = {
        let nav = navigator.clone();
        move |event: MouseEvent| {
            nav.push(&Route::Home);
        }
    };

    let go_about = {
        let nav = navigator.clone();
        move |event: MouseEvent| {
            nav.push(&Route::About);
        }
    };

    let q_anime_update = {
        let q_disp = q_disp.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            q_disp.set(NavbarSearch{query: value});
        })
    };

    let search_anime = {
        let query = (*q_state).query.clone();
        let r_disp = r_disp.clone();
        let nav = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            nav.push(&Route::Loading);
            e.prevent_default();
            let query = query.clone();
            let r_disp = r_disp.clone();
            let nav = nav.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let result = reqwasm::http::Request::get(&format!("https://api.jikan.moe/v4/anime?q={}", query))
                    .send()
                    .await
                    .unwrap()
                    .json::<QueryResult>()
                    .await
                    .unwrap();

                log!(format!("{:?}", &result));
                r_disp.set(result);
                nav.push(&Route::AnimeResult {page: 0});
            });

            log!("Successfully retrieved JSON response!");
            log!("Pushing to navigator...");
        })
    };

    let clear_text = Callback::from(move |event: FocusEvent| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        document
            .query_selector("#q-anime")
            .unwrap()
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .select();
    });

    html! {
        <div class={format!("top_navbar {}", &class_added)}>
            <span class="nb-span-left">
                <a onclick={go_home} style={"cursor: pointer"}><h1 id="main-logo-navbar">{"ANiNFO"}</h1></a>
                // <button class="nb-btn" onclick={go_home}>{"Home"}</button>
                <button id="navbar-about-btn" class="nb-btn" onclick={go_about}>{"About"}</button>
            </span>

            <span class="nb-span-right">
                <form onsubmit={search_anime} method="post">
                <label for="q-anime"><span id="label-q-anime">{"Search"}</span>
                <input class={class_added} id="q-anime" type="text" oninput={q_anime_update} onfocus={clear_text}/> </label>
                <button class="nb-btn-submit" type="submit">{"Submit"}</button>
                </form>
                <button class="nb-btn-theme" onclick={theme_toggle_onchange}>{theme_tog_label}</button>

                if &(*app_ctx).language == &Language::EN {
                    <button class="nb-btn-theme" onclick={language_toggle_onchange}><b>{"EN"}</b><span style="font-weight: 100">{" | JP"}</span></button>
                } else {
                <button class="nb-btn-theme" onclick={language_toggle_onchange}><span style="font-weight: 100">{"EN | "}</span><b>{"JP"}</b></button>}
                
            </span>
        </div>
    }
}
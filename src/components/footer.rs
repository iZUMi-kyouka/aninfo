use yew::prelude::*;
use yew_router::prelude::*;
use crate::{components::appcontext_provider::AppContext, utils::handle_theme, Route};

#[function_component(Footer)]
pub fn footer() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();

    let back_to_top = |e: MouseEvent| {
        let window = web_sys::window().unwrap();
        window.scroll_to_with_x_and_y(0f64, 0f64);
    };

    html! {
        <div class={format!("{} footer", handle_theme(&app_ctx))}>
        <div class="footer-left">
        <p>{"Made with "}<span class="love-footer">{"❤"}</span>{" by"}<b>{" iZUMi"}</b></p>
        <p><a onclick={move |_| nav.push(&Route::PrivacyPolicy)}>{"Privacy Policy"}</a></p>
        </div>
        
        <div class="footer-right">
            <button class={format!("nb-btn-theme {}", handle_theme(&app_ctx))} onclick={back_to_top}>{"Top"}</button>
        </div>
        </div>
    }
}
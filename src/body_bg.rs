use yew::prelude::*;

use crate::{components::appcontext_provider::AppContext, utils::handle_theme};

#[function_component(BodyWrapper)]
pub fn body_wrapper() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();
    html! {
        <div class={format!("body-wrapper {}", handle_theme(&app_ctx))}>
        </div>
    }
}
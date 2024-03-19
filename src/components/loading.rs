use yew::prelude::*;

use crate::{components::appcontext_provider::AppContext, utils::handle_theme};

#[function_component(Loading)]
pub fn loading() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();

    html! {
        <div class="loading-wrapper"><progress class="pure-material-progress-circular"/></div>
    }
}
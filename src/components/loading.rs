use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::appcontext_provider::AppContext, utils::handle_theme};

#[function_component(Loading)]
pub fn loading() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();

    {
        let app_ctx = app_ctx.clone();
        use_effect_with(app_ctx.clone(), move |_| {
            if !(*app_ctx).loading_page {
                nav.back();
            }
        });
    
    }

    html! {
        <div class="loading-wrapper"><progress class="pure-material-progress-circular"/></div>
    }
}
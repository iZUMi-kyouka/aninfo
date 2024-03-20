use yew::prelude::*;

use crate::{components::appcontext_provider::{AppContext, Theme}, utils::handle_theme};

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();

    let theme = handle_theme(&app_ctx);
    
    use_effect(|| {
        web_sys::window().unwrap().document().unwrap().set_title(&format!("ANiNFO: {}", "About"));
    });

    html! {
        <div class={format!("about {}", theme)}>
            <h2>{"About ANiNFO"}</h2>
            <p>{"ANiNFO is an anime website offering users the service of retrieving information about anime."}</p>
            <h3>{"Why do we exist?"}</h3>
            <p>{"We strive to offer a one-stop service where users can get information about a specific anime that is hassle-free.
            This project also serves as a prove that Rust is production-ready!"}</p>
            <h3>{"Technical Information"}</h3>
            <p>{"This service is a mostly frontend-only service written in Rust using the Yew.rs framework. The source code are then compiled to WASM using Trunk.
            The service is offered by Jikan REST"}</p>
        </div>
    }
}
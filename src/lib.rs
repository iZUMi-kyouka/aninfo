use gloo::console::log;
use serde_json::from_str;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::back_btn::BackBtn;
use crate::components::footer::Footer;
use crate::{body_bg::BodyWrapper, components::{animeobjcontext_provider::AnimeObjContextProvider, appcontext_provider::{self, AppContextProvider}, stores::TestObj, top_navbar::TopNavbar}};
use crate::components::routes::*;
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;


mod utils;
mod components;
mod body_bg;


#[function_component(App)]
pub fn app() -> Html {

    use_effect(move || {
        let document = gloo::utils::document();
        let document_cloned = document.clone();

        let listener = EventListener::new(&document_cloned, "keydown", move |e| {
            let e = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();

            let key = e.key();
            let alt = e.alt_key();

            let window = web_sys::window().unwrap();
            let doc = window.document().unwrap();
            let active_element = doc
                .active_element()
                .unwrap();
            
            if  (&key == "/") && (active_element != document.query_selector("#q-anime").unwrap().unwrap()) && (alt) {

                let window = web_sys::window().expect("There should be a windows.");
                let document = window.document().expect("There should be a document in this windows.");
                document
                    .query_selector("#q-anime")
                    .unwrap()
                    .unwrap()
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
                    .focus()
                    .unwrap();

                    document
                    .query_selector("#q-anime")
                    .unwrap()
                    .unwrap()
                    .dyn_ref::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");
            } 
        });

        || drop(listener)
    });
    
    html!{
        <AppContextProvider>
            <AnimeObjContextProvider>
            <BrowserRouter>
            <div class="router-wrapper">
                <TopNavbar/>
                <Switch<Route> render={switch}/>
            </div>
            // <BackBtn/>
            <Footer/>
            </BrowserRouter>
            </AnimeObjContextProvider>
        </AppContextProvider>
    }
}
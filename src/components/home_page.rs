use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {

    use_effect(|| {
        web_sys::window().unwrap().document().unwrap().set_title(&format!("ANiNFO: {}", "Home"));
    });

    html! {
        <h2>{"Placeholder Home Page."}</h2>
    }
}
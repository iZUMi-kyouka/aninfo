use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(BackBtn)]
pub fn back_btn() -> Html {
    let nav = use_navigator().unwrap();
    let back = move |e: MouseEvent| {
        nav.back();
    };
    
    html! {
        <div class="back-btn">
        <button onclick={back}>{"Back"}</button>
        </div>
    }
}
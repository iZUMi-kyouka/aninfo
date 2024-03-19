use yew::prelude::*;
use yewdux::prelude::*;

use crate::{components::appcontext_provider::AppContext, utils::handle_theme};

use super::stores::Char;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub char_obj: Char
}

#[function_component(CharCard)]
pub fn char_card(props: &Props) -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();

    html! {
        <div class={format!("char-card {}", handle_theme(&app_ctx))}>
        <img src={props.char_obj.character.images.webp.image_url.clone()} alt="char_pic"/>
        <div class="chard-card-name">{props.char_obj.character.name.clone()}</div>
        <div class="chard-card-role">{props.char_obj.role.clone()}</div>
        </div>
    }   
}
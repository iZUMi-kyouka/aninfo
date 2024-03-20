use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::components::{appcontext_provider::AppContext, stores::NavbarSearch};
use crate::components::routes::Route;
use crate::utils::handle_theme;
use crate::components::stores::*;

#[function_component(NAME)]
pub fn name() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();
    let (nbs_state, nbs_disp) = use_store::<NavbarSearch>();

    {
        let app_ctx = app_ctx.clone();
        let nav = nav.clone();
        let (nbs_state, nbs_disp) = (nbs_state.clone(), nbs_disp.clone());
        use_effect_with((), |_| {
            web_sys::window().unwrap().document().unwrap().set_title(&format!("ANiNFO: {}", "Home"));
        });
    }

    html!()
}
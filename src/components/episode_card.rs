use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::components::appcontext_provider::Language;
use crate::components::{appcontext_provider::AppContext, stores::NavbarSearch};
use crate::components::routes::Route;
use crate::utils::handle_theme;

use super::stores::AnimeEpisode;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub anime_eps: AnimeEpisode
}

#[function_component(EpisodeCard)]
pub fn name(props: &Props) -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();
    let (nbs_state, nbs_disp) = use_store::<NavbarSearch>();
    let title_rendered = use_state(|| "".to_string());
    let eps = (props.anime_eps).clone();

    {
        let eps = eps.clone();
        let app_ctx = app_ctx.clone();
        let nav = nav.clone();
        let (nbs_state, nbs_disp) = (nbs_state.clone(), nbs_disp.clone());
        let title_rendered = title_rendered.clone();
        use_effect_with(app_ctx.clone(), move |_| {
            match &((*app_ctx).language) {
                &Language::EN => title_rendered.set((eps.title).clone()),
                &Language::JP => {
                    if let Some(t) = (eps.title_japanese).clone() {
                        title_rendered.set(t)
                    } else if let Some(t) = (eps.title_romanji).clone() {
                        title_rendered.set(t)
                    } else {
                        title_rendered.set((eps.title).clone())
                    }
                }
            }
    });
    }

    html!(
        <div class={format!("eps-card {}-emp", handle_theme(&app_ctx))}>
            <div id="eps-no">
            <h3>{props.anime_eps.mal_id}</h3>
            </div>
            <div id="eps-info">
                <div id="eps-title"><a target={"_blank"} href={
                    (props.anime_eps.url).clone().unwrap_or((props.anime_eps.forum_url).clone().unwrap_or("".to_string()))}>{(*title_rendered).clone()
                    }</a></div>
                <div id="eps-meta"></div>
            </div>
        </div>
    )
}
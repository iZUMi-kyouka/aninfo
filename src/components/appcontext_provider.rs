use std::rc::Rc;

use yew::{prelude::*};

use crate::utils::handle_theme;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Theme {
    Light(&'static str),
    Dark(&'static str),
    #[default]
    Auto
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Language {
    #[default]
    EN,
    JP
}

pub const LIGHT_THEME: Theme = Theme::Light("light");
pub const DARK_THEME: Theme = Theme::Dark("dark");
pub const AUTO_THEME: Theme = Theme::Auto;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppCtx {
    pub theme: Theme,
    pub language: Language,
    pub cur_page: u8,
    pub loading_page: bool,
    pub nsfw: bool,
}

impl Reducible for AppCtx {
    type Action = AppCtx;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }

}

impl AppCtx {
    pub fn update_theme_into(&self, new_theme: Theme) -> AppCtx{
        AppCtx {
            theme: new_theme,
            language: (&self.language).clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
        }
    }

    pub fn update_language_into(&self, new_lang: Language) -> AppCtx {
        AppCtx {
            theme: (&self.theme).clone(),
            language: new_lang,
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
        }
    }

    pub fn update_page_into(&self, new_page: u8) -> AppCtx {
        AppCtx {
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: new_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
        }
    }

    pub fn update_loading_page_into(&self, new_status: bool) -> AppCtx {
        AppCtx {
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: new_status,
            nsfw: (&self).nsfw,
        }
    }

    pub fn update_nsfw_into(&self, new_status: bool) -> AppCtx {
        AppCtx {
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: new_status,
        }
    }
}

pub type AppContext = UseReducerHandle<AppCtx>;

#[derive(Properties, Debug, PartialEq)]
pub struct AppContextProviderProps {
    #[prop_or_default]
    pub children: Html
}

#[function_component(AppContextProvider)]
pub fn app(props: &AppContextProviderProps) -> Html {
    let app_ctx = use_reducer(|| AppCtx {
        theme: DARK_THEME,
        language: Language::EN,
        cur_page: 1,
        loading_page: false,
        nsfw: false,
    });

    let app_ctx_cloned = app_ctx.clone();

    html! {
        <ContextProvider<AppContext> context={app_ctx}>
            <div class={format!("wrapper {}", handle_theme(&app_ctx_cloned))}>
            {props.children.clone()}
            </div>
        </ContextProvider<AppContext>>
    }
}
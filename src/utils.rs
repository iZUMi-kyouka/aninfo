use crate::components::appcontext_provider::{AppContext, Theme};

pub fn handle_theme(app_ctx: &AppContext) -> &'static str {
    if let &Theme::Light(class) = &((*app_ctx).theme) {
        class
    } else if let &Theme::Dark(class) = &((*app_ctx).theme) {
        class
    } else {
        ""
    }
}
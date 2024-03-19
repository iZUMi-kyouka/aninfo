use std::rc::Rc;
use yew::prelude::*;

use super::stores::AnimeObj;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AnimeObjCtx {
    pub anime_obj: Option<AnimeObj>
}

impl Reducible for AnimeObjCtx {
    type Action = AnimeObj;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::new(Self{anime_obj: Some(action)})
    }
}

pub type AnimeObjContext = UseReducerHandle<AnimeObjCtx>;

#[derive(Properties, Debug, PartialEq)]
pub struct AnimeObjProviderProps {
    #[prop_or_default]
    pub children: Html
}

#[function_component(AnimeObjContextProvider)]
pub fn provide(props: &AnimeObjProviderProps) -> Html {
    let ao_ctx = use_reducer(|| AnimeObjCtx {
        anime_obj: None
    });

    html! {
        <ContextProvider<AnimeObjContext> context={ao_ctx}>
            {props.children.clone()}
        </ContextProvider<AnimeObjContext>>
    }
}
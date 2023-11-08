use dioxus::prelude::{*, SvgAttributes};
use dioxus_router::prelude::*;

use crate::Apps::LessonLab::Routing;

pub fn App(cx: Scope) -> Element {
    render! { 
        Router::<Routing::Route> {}
    }
}
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::{*, SvgAttributes}, html::hr, html::link};
use dioxus_router::prelude::*;
use crate::Apps::LessonLab::Entry::View::{menu, upload};
use crate::Apps::LessonLab::routing;

pub fn App(cx: Scope) -> Element {
    render! { 
        // a {
        //     href: "/upload",
        //     "text"
        // }
        Router::<routing::Route> {}
        // upload::Upload {}
    }
}
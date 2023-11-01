// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::{*, SvgAttributes}, html::hr, html::link};
use dioxus_router::prelude::*;
use crate::Apps::LessonLab::Entry::View::{Menu, Upload};
use crate::Apps::LessonLab::Routing;

pub fn App(cx: Scope) -> Element {
    render! { 
        Router::<Routing::Route> {}
    }
}
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*, html::hr};

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! { 
        div { "Test" }
    })
}
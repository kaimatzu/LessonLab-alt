#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Header(cx: Scope, title: String) -> Element {
	cx.render(rsx! {
		style { "../assets/style.css" }, 
		h1 { "{title}" }
	})
}
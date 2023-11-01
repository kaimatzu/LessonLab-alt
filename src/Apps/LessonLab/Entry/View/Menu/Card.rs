
#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Card(cx: Scope, title: String, desc: String) -> Element {
	cx.render(rsx! {
		strong { "{title}" }
		br {}
		p { "{desc}" }
	})
}
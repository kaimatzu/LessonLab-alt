#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Button(cx: Scope, text: String) -> Element {
	cx.render(rsx!{
		button { "{text}" }
	})
}
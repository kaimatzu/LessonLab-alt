#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Bar(cx: Scope, text: String) -> Element {
	cx.render(rsx! {
		div { class: "bar",
			"{text}"
		}
	})
}
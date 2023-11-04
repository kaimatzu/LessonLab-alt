
#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Card<'a>(cx: Scope, title: &'a str, desc: &'a str) -> Element<'a> {
	cx.render(rsx! {
		div { class: "card",
			width: "400px",
			height: "300px",
			strong { "style": "font-size: 32px;",
				"{title}" }
			br {}
			p { "{desc}" }
		}
	})
}
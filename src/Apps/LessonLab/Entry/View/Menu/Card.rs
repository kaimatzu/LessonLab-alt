
#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Card(cx: Scope, title: String, desc: String) -> Element {
	cx.render(rsx! { style { include_str!("../../../../../../assets/style.css") },
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
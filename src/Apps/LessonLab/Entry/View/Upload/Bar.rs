#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Bar(cx: Scope, text: String) -> Element {
	cx.render(rsx! { style { include_str!("../../../../../../assets/style.css") }
		div { class: "bar", width: "500px", height: "35px",
			// text { "{text}" }
			"{text}"
		}
	})
}
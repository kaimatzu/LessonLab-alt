#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Button(cx: Scope, text: String, classname: String, idname: String) -> Element {
	cx.render(rsx!{
		button { "style": "right-buttons", class: "{classname}", id: "{idname}",
			"{text}"
		}
	})
}
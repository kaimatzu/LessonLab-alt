#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ComponentTemplates::Bar::Bar::Bar;

#[inline_props]
pub fn FileContainer(cx: Scope, section: String, num: i32) -> Element {
	let filename = "this a filename";	
	let mut num = use_state(cx, || 3);
	cx.render(rsx! {
		div { class: "filetype-container",
		h1 { "style": "font-size: 36px;", "{section}" }
			// list of files here
			// for loop
			for i in 0..*num.get() {
				Bar { text: filename.to_string() }
			}
		}
	})
}
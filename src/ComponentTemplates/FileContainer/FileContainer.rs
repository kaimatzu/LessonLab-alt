#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ComponentTemplates::Bar::Bar::Bar;

#[inline_props]
pub fn FileContainer<'a>(cx: Scope, section: String, file_amount: &'a i32) -> Element<'a> {
	let filename = "this a filename";	
	// let mut num = use_state(cx, || 3);
	cx.render(rsx! {
		div { class: "filetype-container",
		h1 { "style": "font-size: 36px;", "{section}" }
			// list of files here
			// for loop
			for i in 0..**file_amount {
				Bar { text: filename.to_string() }
			}
		}
	})
}
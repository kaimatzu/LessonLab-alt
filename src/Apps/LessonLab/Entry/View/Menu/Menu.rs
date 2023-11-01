#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;
use crate::Apps::LessonLab::routing::Route;

pub fn Menu(cx: Scope) -> Element {
	let mut text = use_state(cx, || "Hello World");
	let num = 5;
	cx.render(rsx!{ style { include_str!("../../../../../../assets/style.css") },
		h1 { "LessonLab" }
		div {
			h2 { "style": "display: flexbox;",
				"Dashboard" 
			}
			Link {
				to: Route::Upload {},
				button { class: "primary-button", "+ New Material"}
			}
			// button { class: "primary-button",
			// 	onclick: move |_| {/* Write some process here */},
			// 	"+ New Material"
			// }
		}

		for i in 0..num {
			{}
		}
		// list of items here
	})
}

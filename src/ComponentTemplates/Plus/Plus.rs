#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Plus(cx: Scope) -> Element {
	cx.render(rsx! {
		div { class: "plus",
			"+"
		}
		// div { class: "plus",
		// 	onclick: move |_| println!("Clicked"), "+"
		// }
	})
}
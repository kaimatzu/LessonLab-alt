#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Plus(cx: Scope/*, onClick: () */) -> Element {
	cx.render(rsx! { style { include_str!("../../../../../../assets/style.css") }
		"+"
		// div { class: "plus",
		// 	onclick: move |_| println!("Clicked"), "+"
		// }
	})
}
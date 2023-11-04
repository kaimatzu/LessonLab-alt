#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Plus<'a>(cx: Scope, on_click: EventHandler<'a, MouseEvent>) -> Element {
	cx.render(rsx! {
		div {
			class: "plus",
			onclick: move |e| on_click.call(e),
			"+"
		}

		// div { class: "plus",
		// 	onclick: move |_| println!("Clicked"), "+"
		// }
	})
}
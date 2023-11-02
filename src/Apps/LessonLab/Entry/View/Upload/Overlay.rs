#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Overlay(cx: Scope) -> Element {
	cx.render(rsx! { style { include_str!("../../../../../../assets/style.css") }
		div {
			
		}
	})
}
#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;

pub fn Menu(cx: Scope) -> Element {
	render! {
		h1 { "Menu" }
		Link {
			to: "https://stackoverflow.com/", "link"
		}
	}
}

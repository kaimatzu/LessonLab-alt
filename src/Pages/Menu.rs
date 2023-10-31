#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn Menu(cx: Scope) -> Element {
	render! {
		h1 { "Menu" }
		Link {
			to: "/upload", "upload"
		}
	}
}

pub struct MenuView {
	// make functions to render in `Menu` functional component
}

pub struct MenuModel {
	// data
}

pub struct MenuController {
	// integrated with python here
}
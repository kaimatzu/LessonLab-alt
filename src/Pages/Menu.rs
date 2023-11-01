#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::Components::Header::Header;

pub fn Menu(cx: Scope) -> Element {
	cx.render(rsx!{ style { "./assets/style.css" }, 
		Header { title: "LessonLab".to_string() }
		Link {
			to: "/upload", "Upload Page"
		}
	})
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
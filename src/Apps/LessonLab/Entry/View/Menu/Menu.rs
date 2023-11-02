#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;
use crate::Apps::LessonLab::routing::Route;
use crate::Apps::LessonLab::Entry::View::Menu::Card::Card;
use crate::ComponentTemplates::Header::Header::Header;

pub fn Menu(cx: Scope) -> Element {
	let mut num = use_state(cx, || 5);
	// let nav = use_navigator(cx);
	cx.render(rsx!{ style { include_str!("../../../../../../assets/style.css") },
		Header { title: "LessonLab".to_string() }
		main { id: "menu-main",
			div { id: "dashboard-text-container",
				h2 { id: "dashboard-text",
					"Dashboard" 
				}
				Link {
					to: Route::Upload {},
					button { class: "primary-button", id: "new-material-button", "+ New Material"}
				}

				// button { class: "primary-button", id: "new-material-button",
				// 	width: "198.25px", height: "52px",
				// 	onclick: move |_| { nav.push(Route::Upload {  }) },
				// 	"+ New Material"
				// }
			}
			div { "style": "display: flex; flex-wrap: wrap; justify-content: space-between;",
				for i in 0..*num.get() {
					Card { title: "Title".to_string(), desc: "description".to_string() }
				}
			}
		}
		// list of items here
	})
}
			// button { class: "primary-button",
			// 	onclick: move |_| {/* Write some process here */},
			// 	"+ New Material"
			// }
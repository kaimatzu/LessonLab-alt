#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;
use crate::Apps::LessonLab::routing::Route;
use crate::ComponentTemplates::Card::Card::Card;
use crate::ComponentTemplates::Header::Header::Header;
use crate::ComponentTemplates::Button::Button::Button;

pub fn Menu(cx: Scope) -> Element {
	let mut num = use_state(cx, || 5);
	// let nav = use_navigator(cx);
	cx.render(rsx!{
		style {
			include_str!("styles/header-style.css")
			include_str!("styles/card-style.css")
			include_str!("styles/button-style.css")
			include_str!("styles/dashboard-text-style.css")
			include_str!("styles/body-style.css")
			include_str!("styles/main-style.css")
			include_str!("styles/logo-style.css")
			include_str!("styles/title-style.css")
		},
		Header { title: "LessonLab".to_string() }
		main {
			div { class: "dashboard-text-container",
				h2 { class: "dashboard-text",
					"Dashboard" 
				}
				Link {
					to: Route::Upload {},
					Button { 
						text: "+ New Material",
						classname: "primary-button",
						idname: ""
					}
				}

				// button { class: "primary-button", id: "new-material-button",
				// 	width: "198.25px", height: "52px",
				// 	onclick: move |_| nav.push(Route::Upload {}),
				// 	"+ New Material"
				// }
			}
			div { "style": "display: flex; flex-wrap: wrap; justify-content: space-between;",
				for i in 0..*num.get() {
					Card {
						title: "Title",
						desc: "description"
					}
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
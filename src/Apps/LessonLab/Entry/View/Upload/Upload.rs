#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;

use crate::Apps::LessonLab::routing::Route;

use crate::ComponentTemplates::Header::Header::Header;
use crate::ComponentTemplates::Button::Button::Button;
use crate::ComponentTemplates::Plus::Plus::Plus;
use crate::ComponentTemplates::FileContainer::FileContainer::FileContainer;

pub fn Upload(cx: Scope) -> Element {
	let mut num = use_state(cx, || 5);
	render! {
		style {
			include_str!("styles/header-style.css")
			include_str!("styles/body-style.css")
			include_str!("styles/logo-style.css")
			include_str!("styles/title-style.css")
			include_str!("styles/bar-style.css")
			include_str!("styles/plus-style.css")
			include_str!("styles/main-style.css")
			include_str!("styles/primary-button-style.css")
			include_str!("styles/secondary-button-style.css")
			include_str!("styles/lower-right-button-style.css")
		},
		Header { title: "LessonLab".to_string() }
		main {
			div { "style": "display: flex; justify-content: flex-end;",
				h2 { "Upload File" } // page name
			}
			div { "style": "padding-bottom: 60px;",
				FileContainer { section: "PDF".to_string(), num: *num.get() }
				FileContainer { section: "URL".to_string(), num: *num.get() }
				FileContainer { section: "Text".to_string(), num: *num.get() }
			}
			div { "style": "display: flex; gap: 10px; justify-content: flex-end;",
				Link { to: Route::Menu {},
					Button {
						classname: "secondary-button".to_string(),
						idname: "right-buttons".to_string(),
						text: "Cancel".to_string()
					}
				}
				Link { to: "/",
					Button {
						classname: "primary-button".to_string(),
						idname: "right-buttons".to_string(),
						text: "Next".to_string()
					}
				}
				Plus {}
				// Plus { onClick }
			}
		}
	}
}
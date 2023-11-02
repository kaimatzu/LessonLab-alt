#![allow(non_snake_case)]

use dioxus_router::prelude::Link;
use dioxus::prelude::*;

use crate::Apps::LessonLab::Entry::View::Upload::FileContainer::FileContainer;
use crate::Apps::LessonLab::Entry::View::Upload::Plus::Plus;
use crate::ComponentTemplates::Button::Button::Button;
use crate::Apps::LessonLab::routing::Route;

pub fn Body(cx: Scope) -> Element {
	let mut num = 0;
	// pub fn onClick(&cx: Scope<'_>) -> VNode<'_, _> {
	// 	render! {
	// 		{}
	// 	}
	// }
	cx.render(rsx! { style { include_str!("../../../../../../assets/style.css") }
		div { id: "menu-main",
			div { "style": "display: flex; justify-content: flex-end;",
				h2 { "Upload File" }
			}
			div { "style": "padding-bottom: 60px;",
				FileContainer { section: "PDF".to_string(), num: num }
				FileContainer { section: "URL".to_string(), num: num }
				FileContainer { section: "Text".to_string(), num: num }
			}
			div { "style": "display: flex; gap: 10px; justify-content: flex-end;",
				Link { to: Route::Menu {},
					Button { classname: "secondary-button".to_string(), idname: "right-buttons".to_string(),
						text: "Cancel".to_string()
					}
				}
				Link { to: "/",
					Button { classname: "primary-button".to_string(), idname: "right-buttons".to_string(),
						text: "Next".to_string()
					}
				}
				Plus {}
				// Plus { onClick }
			}
		}

	})
}
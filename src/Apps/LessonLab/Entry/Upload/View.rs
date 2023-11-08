#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;

use crate::Apps::LessonLab::Routing::Route;

use crate::ComponentTemplates::Header::Header::Header;
use crate::ComponentTemplates::Button::Button::Button;
use crate::ComponentTemplates::Overlay::Overlay::Overlay;
use crate::ComponentTemplates::Plus::Plus::Plus;
use crate::ComponentTemplates::FileContainer::FileContainer::FileContainer;

use crate::Apps::LessonLab::Entry::Upload::ViewModel::*;

#[inline_props]
pub fn UploadComponent<'a>(
	cx: Scope, 
	pdf_amount: &'a i32, 
	url_amount: &'a i32, 
	text_amount: &'a i32,
	overlay_visibility: &'a bool,
	on_overlay_button_click: EventHandler<'a, MouseEvent>) -> Element<'a> {

	let mut is_visible = use_state(cx, || false);

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
			include_str!("styles/overlay-style.css")
		},

		Overlay { 
			is_visible: **overlay_visibility,
			on_click: move |event: MouseEvent| on_overlay_button_click.call(event), 
		}

		Header { title: "LessonLab" }

		main {
			div { "style": "display: flex; justify-content: flex-end;",
				h2 { "Upload File" } // page name
			}
			div { "style": "padding-bottom: 60px;",
				FileContainer { section: "PDF".to_string(), file_amount: pdf_amount }
				FileContainer { section: "URL".to_string(), file_amount: url_amount }
				FileContainer { section: "Text".to_string(), file_amount: text_amount }
			}
			div { "style": "display: flex; gap: 10px; justify-content: flex-end;",
				Link { to: Route::Menu {},
					Button {
						classname: "secondary-button",
						idname: "right-buttons",
						text: "Cancel"
					}
				}
				Link { to: "/",
					Button {
						classname: "primary-button",
						idname: "right-buttons",
						text: "Next"
					}
				}
				// When user clicks plus button 
				// opens a dialogue that will let the user choose what type of file to upload
				// if PDF => open file dialogue
				// if URL => open text field for url
				// if Text => open text area

				Plus {
					classname: "plus",
					on_click: move |event: MouseEvent| on_overlay_button_click.call(event),
				}
				// Plus { onClick }
			}
		}
	}
}

pub fn Upload(cx: Scope) -> Element {
    let (pdf_amount_state, 
		url_amount_state, 
		text_amount_state, 
		overlay_visibility_state,
		visibility_event) = ViewModel(cx);

    cx.render(rsx!{
        UploadComponent { 
			pdf_amount: pdf_amount_state, 
			url_amount: url_amount_state, 
			text_amount: text_amount_state,
			overlay_visibility: overlay_visibility_state,
			on_overlay_button_click: visibility_event,
		},
    })
}
#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;
use crate::Apps::LessonLab::Routing::Route;
use crate::ComponentTemplates::Card::Card::Card;
use crate::ComponentTemplates::Header::Header::Header;
use crate::ComponentTemplates::Button::Button::Button;

use crate::Apps::LessonLab::Entry::Menu::ViewModel::*;

#[inline_props]
pub fn MenuComponent<'a>(
    cx: Scope, 
    item_amount: &'a i32, 
    on_add_button_click: EventHandler<'a, MouseEvent>) -> Element<'a> {

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
			include_str!("styles/cards-container-style.css")
        },
        Header { title: "LessonLab" }
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
						idname: "",
					}
				}
                
            }
            div { 
                class: "cards-container",
                for i in 0..**item_amount {
                    Card { title: "Title", desc: "description" }
                }
            }
            
            // Remove this puhon. For testing only
            button { class: "primary-button", id: "new-material-button",
                width: "198.25px", height: "52px",
                onclick: move |event: MouseEvent| on_add_button_click.call(event),
                "+ New"
            }
        }
        // list of items here
    })
}

pub fn Menu(cx: Scope) -> Element {
    let (items_state, add_event) = ViewModel(cx);
    cx.render(rsx!{
        MenuComponent { 
            item_amount: items_state, 
            on_add_button_click: add_event 
        },
    })
}
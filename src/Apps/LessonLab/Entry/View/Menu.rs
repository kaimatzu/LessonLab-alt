#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;
use crate::Apps::LessonLab::Routing::Route;

#[inline_props]
pub fn Menu(cx: Scope) -> Element {
	render! {
		h1 { "Menu" }
		Link {
			to: Route::Upload {}, "link"
		}
	}
}

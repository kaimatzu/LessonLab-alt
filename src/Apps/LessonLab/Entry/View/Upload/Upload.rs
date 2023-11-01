#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;
use crate::Apps::LessonLab::routing::Route;

pub fn Upload(cx: Scope) -> Element {
	render! {
		h1 { "Upload" }
		Link {
			to: Route::Menu{}, "Cancel"
		}
	}
}

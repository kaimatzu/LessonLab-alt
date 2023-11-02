#![allow(non_snake_case)]

use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;

use crate::Apps::LessonLab::Entry::View::Upload::FileContainer::FileContainer;
use crate::Apps::LessonLab::routing::Route;
use crate::ComponentTemplates::Header::Header::Header;
use crate::Apps::LessonLab::Entry::View::Upload::Body::Body;

pub fn Upload(cx: Scope) -> Element {
	render! { style { include_str!("../../../../../../assets/style.css") }
		Header { title: "LessonLab".to_string() }
		Body {}
	}
}
use dioxus::{prelude::*, html::hr};
use dioxus_router::prelude::*;

use crate::Apps::LessonLab::Entry::View::Menu::Menu;
use crate::Apps::LessonLab::Entry::View::Upload::Upload;
// ANCHOR: router
#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
	#[route("/")]
	Menu {},
	#[route("/upload")]
	Upload {}
}
// ANCHOR_END: router
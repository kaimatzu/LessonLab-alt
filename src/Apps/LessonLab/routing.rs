use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Apps::LessonLab::Entry::Menu::View::Menu;
use crate::Apps::LessonLab::Entry::Upload::View::Upload;
// ANCHOR: router
#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
	#[route("/")]
	Menu {},
	#[route("/upload")]
	Upload {}
}
// ANCHOR_END: router
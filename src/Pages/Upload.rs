#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn Upload(cx: Scope) -> Element {
	cx.render(rsx! {
		h1 { "Upload" }
		Link {
			to: "/", "Main Menu"
		}
	})
}

pub struct UploadView {
	// make functions to render in `Upload` functional component
}

pub struct UploadModel {
	// data
}

pub struct UploadController {
	// integrate with python here
}
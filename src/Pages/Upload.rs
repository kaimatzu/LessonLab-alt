#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Upload(cx: Scope) -> Element {
	render! {
		h1 { "Upload" }
	}
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
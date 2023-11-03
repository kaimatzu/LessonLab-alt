#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Header(cx: Scope, title: String) -> Element {
	cx.render(rsx! { 
		header {
			img {
				width: "174px", height: "149px",
				src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4"
			}
			p { class: "title",
				"LessonLab"
			}
		}
	})
}
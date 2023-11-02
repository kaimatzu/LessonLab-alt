#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Header(cx: Scope, title: String) -> Element {
<<<<<<< HEAD
	cx.render(rsx! {
=======
	cx.render(rsx! { style { include_str!("../../../assets/style.css") }
>>>>>>> CU-86ctu77m2_Implement-UI-Upload-File_Hans-Duran
		header { id: "menu-header",
			img {
				width: "174px", height: "149px",
				src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4"
			}
			p { class: "title",
<<<<<<< HEAD
				"LessonLabs"
=======
				"LessonLab"
>>>>>>> CU-86ctu77m2_Implement-UI-Upload-File_Hans-Duran
			}
		}
	})
}
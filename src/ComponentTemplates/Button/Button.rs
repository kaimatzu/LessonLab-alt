#![allow(non_snake_case)]

use dioxus::{prelude::*, html::button};

#[derive(Props, Default)]
pub struct ButtonProps<'a> {
	pub text: &'a str,
	pub classname: Option<&'a str>,
	pub idname: Option<&'a str>,
	pub on_click: Option<EventHandler<'a, MouseEvent>>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps>) -> Element<'a> {
	let classname = cx.props.classname.unwrap_or("");
	let idname = cx.props.idname.unwrap_or("");
	// let option = cx.props.on_click.clone().unwrap_or(EventHandler::default());
	// let future = use_future(cx, on_click.callback, |e| async move {
	// 	on_click.call(e)
	// });
	// let option = cx.props.on_click.clone().unwrap_or(EventHandler::default());
	// let on_click = cx.props.on_click.as_ref().map(|handler| {
	// 	let cloned_handler = handler.clone();
	// 	move |evt| cloned_handler.call(evt);
	// });

	let button_element = if let Some(on_click) = &cx.props.on_click {
		rsx! {
			button {
				class: "{classname}",
				id: "{idname}",
				onclick: move |e| on_click.call(e),
				"{cx.props.text}"
			}
		}

	} else {
		rsx! {
			button {
				class: "{classname}",
				id: "{idname}",
				"{cx.props.text}"
			}
		}
	};
	cx.render(button_element)

	// cx.render(rsx!{
	// 	button { class: "{classname}", id: "{idname}",
	// 		onclick: on_click,
	// 		"{cx.props.text}"
	// 	}
	// })
	// if option == Some(()) {
	// 	cx.render(rsx!{
	// 		button { class: "{classname}", id: "{idname}",
	// 			onclick: move |e| option.unwrap().call(e),
	// 			"{cx.props.text}"
	// 		}
	// 	})
	// } else {
	// 	cx.render(rsx!{
	// 		button { class: "{classname}", id: "{idname}",
	// 			"{cx.props.text}"
	// 		}
	// 	})
	// }
}
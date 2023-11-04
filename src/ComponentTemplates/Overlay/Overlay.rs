use dioxus::prelude::{*, SvgAttributes};

use crate::ComponentTemplates::Button::Button::Button;

#[inline_props]
pub fn Overlay<'a>(cx: Scope, is_visible: bool, on_click: EventHandler<'a, MouseEvent>) -> Element {
	if !cx.props.is_visible {
		return None;
	}

	// 0 = select
	// 1 = PDF
	// 2 = URL
	// 3 = Text
	let mut state = use_state(cx, || 0);
	let num = use_state(cx, || 0);
	if *state.get() == 1 { // PDF
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| on_click.call(e),
							"{*num.get()}"
						}
					}
					div { // 3 buttons
						"style": "display: flex; flex-direction: column; gap: 70px; bottom: 0px; padding: 50px;",
						Button {
							text: "PDF",
							classname: "primary-button overlay-button",
							on_click: move |evt| on_click.call(evt),
						}
						Button {
							text: "URL",
							classname: "primary-button overlay-button",
						}
						Button {
							text: "Text",
							classname: "primary-button overlay-button",
						}
					}

				}
			}
		})
	} else if *state.get() == 2 { // URL
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| on_click.call(e),
							"X"
						}
					}
					div { // 3 buttons
						"style": "display: flex; flex-direction: column; gap: 70px; bottom: 0px; padding: 50px;",
						Button {
							text: "PDF",
							classname: "primary-button overlay-button",
							idname: ""
						}
						Button {
							text: "URL",
							classname: "primary-button overlay-button",
							idname: ""
						}
						Button {
							text: "Text",
							classname: "primary-button overlay-button",
							idname: ""
						}
					}

				}
			}
		})
	} else if *state.get() == 3 {
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| on_click.call(e),
							"X"
						}
					}
					div { // 3 buttons
						"style": "display: flex; flex-direction: column; gap: 70px; bottom: 0px; padding: 50px;",
						Button {
							text: "PDF",
							classname: "primary-button overlay-button",
							idname: ""
						}
						Button {
							text: "URL",
							classname: "primary-button overlay-button",
							idname: ""
						}
						Button {
							text: "Text",
							classname: "primary-button overlay-button",
							idname: ""
						}
					}

				}
			}
		})
	} else { // Choose 
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| on_click.call(e),
							"X"
						}
					}
					div { // 3 buttons
						"style": "display: flex; flex-direction: column; gap: 70px; bottom: 0px; padding: 50px;",
						Button {
							text: "PDF",
							classname: "primary-button overlay-button",
							idname: ""
						}
						Button {
							text: "URL",
							classname: "primary-button overlay-button",
							idname: ""
						}
						Button {
							text: "Text",
							classname: "primary-button overlay-button",
							idname: ""						}
					}

				}
			}
		})
	}
}
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


	if *state.get() == 1 { // PDF
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| {
								state.set(0);
								on_click.call(e)
							},
							"X"
						}
					}

					"Drag and drop area for pdf files"

				}
			}
		})
	} else if *state.get() == 2 { // URL
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| { 
								state.set(0);
								on_click.call(e)
							},
							"X"
						}
					}

					"Url input"

				}
			}
		})
	} else if *state.get() == 3 { // Text
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| {
								state.set(0);
								on_click.call(e)
							},
							"X"
						}
					} // x div

					"Large text area"

				}
			}
		})
	} else { // Choose 
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| {
								state.set(0);
								on_click.call(e)
							},
							"X"
						}
					}
					// 3 buttons
					div {
						"style": "display: flex; flex-direction: column; gap: 70px; bottom: 0px; padding: 50px;",
						Button {
							text: "PDF",
							classname: "primary-button overlay-button",
							idname: "",
							on_click: move |evt| state.set(1),
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
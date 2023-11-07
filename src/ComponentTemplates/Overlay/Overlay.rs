use dioxus::prelude::{*, SvgAttributes};

use crate::ComponentTemplates::Button::Button::Button;
use crate::ComponentTemplates::Plus::Plus::Plus;

#[derive(PartialEq)]
enum State {
	Select,
	Pdf,
	Url,
	Text,
}

enum InnerState {
	Add,
	Edit
}

#[inline_props]
pub fn Overlay<'a>(cx: Scope, is_visible: bool, on_click: EventHandler<'a, MouseEvent>) -> Element {
	if !cx.props.is_visible {
		return None;
	}


	// 0 = select
	// 1 = PDF
	// 2 = URL
	// 3 = Text
	let mut state = use_state(cx, || State::Select);

	// 0 = add new files
	// 1 = edit files
	let mut inner_state = use_state(cx, || InnerState::Add);


	if *state.get() == State::Pdf { // PDF
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| {
								reset_states(state, inner_state);
								on_click.call(e);
							},
							"X"
						}
					}

					
					Plus {
						classname: "plus",
						on_click: move |_| {}
					}
				}
			}
		})
	} else if *state.get() == State::Url { // URL
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| { 
								reset_states(state, inner_state);
								on_click.call(e);
							},
							"X"
						}
					}


					Plus {
						classname: "plus plus-bottom-right",
						on_click: move |_| {}
					}
				}
			}
		})
	} else if *state.get() == State::Text { // Text
		cx.render(rsx! {
			div { class: "overlay",
				div { class: "inner-overlay",
					div { "style": "display: flex;", // x button div
						div { "style": "width: 12px; height: 20px; justify-content: space-between; margin-left: auto",
							onclick: move |e| {
								reset_states(state, inner_state);
								on_click.call(e);
							},
							"X"
						}
					} // x div

					Plus {
						classname: "plus plud-bottom-right",
						on_click: move |_| {}
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
							onclick: move |e| {
								reset_states(state, inner_state);
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
							on_click: move |_| state.set(State::Pdf),
						}
						Button {
							text: "URL",
							classname: "primary-button overlay-button",
							on_click: move |_| state.set(State::Url),
						}
						Button {
							text: "Text",
							classname: "primary-button overlay-button",
							on_click: move |_| state.set(State::Text),
						}
					}

				}
			}
		})
	}
}

fn reset_states<'a>(state: &'a UseState<State>, inner_state: &'a UseState<InnerState>) {
	state.set(State::Select);
	inner_state.set(InnerState::Add);
}
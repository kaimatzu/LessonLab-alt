use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct OverlayProps {
    is_visible: bool,
    // children: Vec<std::rc::Rc<Component>>,
}

pub fn Overlay(cx: Scope<OverlayProps>) -> Element {
	if cx.props.is_visible {
		return None;
	}

	cx.render(rsx! {
		div { class: "overlay",
			"this is the overlay"
		}
	})
}
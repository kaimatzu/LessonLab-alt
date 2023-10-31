#![allow(non_snake_case)]
#![allow(unused)]


// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*, html::hr};
use dioxus_router::prelude::*;

use LessonLab::Components::Header::Header;
use LessonLab::Components::Button::Button;

fn main() {
    // launch the app   
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
	let hello = hello();

	let login = login();

	let mut count = use_state(cx, || 0);
	let counter = counter(count);


    cx.render(rsx! { 
		hello
		login
		counter
    })
}

fn hello<'a>() -> LazyNodes<'a, 'a> {
    let ret = rsx! {
        div { "hello world" }
    };
    ret
}

fn login<'a>() -> LazyNodes<'a, 'a> {
    let ret = rsx! {
        h1 { "Login" }
        form {
            // onsubmit: onsubmit,
            input { r#type: "text", id: "username", name: "username" }
            label { "Username" }
            br {}
            input { r#type: "password", id: "password", name: "password" }
            label { "Password" }
            br {}
            button { "Login" }
        }
    };
    ret
}

fn counter<'a>(mut count: &'a UseState<i32>) -> LazyNodes<'a, 'a> {
    let res = rsx! {
        h1 { "High-Five counter: {count}" }
        button {
            onclick: move |_| {
                // changing the count will cause the component to re-render
                count -= 1
            },
            "Down low!"
        }
        button {
            onclick: move |_| {
                // changing the count will cause the component to re-render
                count += 1
            },
            "Up high!"
        }
    };
    res
}
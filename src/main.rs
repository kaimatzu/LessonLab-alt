#![allow(non_snake_case)]
#![allow(unused)]

use self::Apps::LessonLab::app;
mod Apps;

use dioxus::{prelude::*, html::hr};
// use std::io::ErrorKind::NotFound;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*, html::hr};
use dioxus_router::prelude::*;
// use crate::IntoRoutable::Route;

use LessonLab::Components::Header::Header;
use LessonLab::Components::Button::Button;
use LessonLab::Pages::Upload::Upload;
use LessonLab::Pages::Menu::Menu;

fn main() {
    // launch the app   
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app::App);
}

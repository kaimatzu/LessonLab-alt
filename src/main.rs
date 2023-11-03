#![allow(non_snake_case)]
#![allow(unused)]

use self::Apps::LessonLab::app;
mod Apps;
mod ComponentTemplates;

fn main() {
    // launch the app   
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app::App);
}

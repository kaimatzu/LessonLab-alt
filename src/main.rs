#![allow(non_snake_case)]
#![allow(unused)]

use self::Apps::LessonLab::Application;
mod Apps;
mod ComponentTemplates;

fn main() {
    // launch the app   
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(Application::App);
}

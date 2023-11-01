#![allow(non_snake_case)]
#![allow(unused)]

use self::Apps::LessonLab::App;
mod Apps;

fn main() {
    // launch the app   
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App::App);
}

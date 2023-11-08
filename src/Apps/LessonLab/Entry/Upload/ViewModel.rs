use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;

use crate::Apps::LessonLab::Entry::Upload::Model::Upload;
use crate::Apps::LessonLab::Entry::Upload::View::UploadComponent;

pub trait ViewModel {
    fn get_pdf_amount(&self) -> i32;
    fn get_url_amount(&self) -> i32;
    fn get_text_amount(&self) -> i32;
    fn get_overlay_visibility(&self) -> bool;
}

impl ViewModel for Upload {
    fn get_pdf_amount(&self) -> i32 {
        self.pdf_amount
    }

    fn get_url_amount(&self) -> i32 {
        self.url_amount
    }

    fn get_text_amount(&self) -> i32 {
        self.text_amount
    }

    fn get_overlay_visibility(&self) -> bool {
        self.overlay_visibility
    }
}

pub fn ViewModel<'a>(cx: Scope<'a>) -> (&'a i32, &'a i32, &'a i32, &'a bool, impl FnMut(Event<MouseData>) + 'a) {
    let upload = Upload{
        pdf_amount: 1,
        url_amount: 2,
        text_amount: 4,
        overlay_visibility: false,
    };

    let mut pdf_amount_state = use_state(cx, || upload.get_pdf_amount());
    let mut url_amount_state = use_state(cx, || upload.get_url_amount());
    let mut text_amount_state = use_state(cx, || upload.get_text_amount());
    let mut overlay_visibility_state = use_state(cx, || upload.get_overlay_visibility());
    let visibility_event = move |event: MouseEvent| overlay_visibility_state.set(!*overlay_visibility_state.get());

    (pdf_amount_state, url_amount_state, text_amount_state, overlay_visibility_state, visibility_event)
}


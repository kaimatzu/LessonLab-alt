use dioxus_router::prelude::*;
use dioxus::prelude::*;
pub struct Upload {
    pub pdf_amount: i32,
    pub url_amount: i32,
    pub text_amount: i32,
    pub overlay_visibility: bool
    // Obviously we change these into proper implementations. Most likely Vec<pdf> or some shit like that. We'll see.
}
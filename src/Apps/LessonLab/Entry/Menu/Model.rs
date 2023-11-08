use dioxus_router::prelude::*;
use dioxus::prelude::*;
pub struct Menu {
    pub user: String,
    pub items: i32, 
    // i32 for now but will change it to appropriate data type later (like a vector of items) and we'll 
    // iterate through all of the items using use_ref
    
    // more data tbd
}
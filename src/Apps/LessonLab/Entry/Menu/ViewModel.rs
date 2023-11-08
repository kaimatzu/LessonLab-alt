use dioxus::{prelude::*, html::hr, html::link};
use dioxus_router::prelude::*;

use crate::Apps::LessonLab::Entry::Menu::Model::Menu;
use crate::Apps::LessonLab::Entry::Menu::View::MenuComponent;
pub trait ViewModel {
	fn get_user(&self) -> &str;
    fn get_items(&self) -> i32;
    fn set_user(&mut self, user: &str);
}

impl ViewModel for Menu {
    fn get_user(&self) -> &str {
        self.user.as_str()
    }

    fn get_items(&self) -> i32 {
        self.items
    }

    fn set_user(&mut self, user: &str){
        self.user = String::from(user);
    }
}

pub fn ViewModel<'a, T>(cx: Scope<'a>) -> (&'a i32, impl FnMut(Event<T>) + 'a) {
    let menu = Menu{
        user: String::from("user"),
        items: 5
    };

    let mut items_state = use_state(cx, || menu.get_items());
    let add_event = move |event| { items_state += 1 }; 

    (items_state, add_event)
}


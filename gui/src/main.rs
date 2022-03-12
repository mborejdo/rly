#![recursion_limit="256"]

mod gui;

fn main() {
    yew::start_app::<gui::Model>();
}
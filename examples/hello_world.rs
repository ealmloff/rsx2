use rsx2::rsx2;
use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx2! {
        "Give me buttons!!!"
    }
}

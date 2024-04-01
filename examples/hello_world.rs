use dioxus::prelude::*;
use rsx2::rsx2;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx2! {
        "Give me buttons!"
    }
}

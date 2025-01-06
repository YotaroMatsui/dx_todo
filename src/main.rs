use console_log::init_with_level;
use dioxus::prelude::*;

mod backend;
mod components;


use components::TaskView;
static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(components::NavBar)]
    #[route("/")]
    TaskView{},
}

fn main() {
    init_with_level(log::Level::Info).expect("Failed to initialize logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

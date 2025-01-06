use dioxus::prelude::*;

mod backend;
mod components;

use components::TaskView;
static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(components::NavBar)]
    #[route("/")]
    TaskView {},
}

fn main() {
    dioxus::logger::initialize_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        SuspenseBoundary { fallback: |_| rsx! { "Loading..." }, Router::<Route> {} }
    }
}

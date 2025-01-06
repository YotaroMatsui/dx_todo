use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "nav-bar",
            h1 { "Todo App" }
        }
        Outlet::<Route> {}
    }
}

mod components;
use dioxus::prelude::*;
use components::categories::{Category, BlogCategory};
use components::about::About;
use components::blog::Blog;
use components::navbar::{Navbar};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/category/:category")]
    Category { category: BlogCategory },
    #[route("/blog/:category/:name")]
    Blog { category: BlogCategory, name: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        "Hello! This page is still under construction. Please come back later to learn more!"
    }
}


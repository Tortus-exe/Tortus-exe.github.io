mod components;
use dioxus::prelude::*;
use components::{
    categories::{Category, BlogCategory},
    about::About,
    blog::Blog,
    navbar::Navbar,
    home::Home,
    articles::Articles,
    skills::Skills,
    projects::Projects,
    contact::Contact};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[layout(Navbar)]
        #[route("/about")]
        About {},
        #[route("/skills")]
        Skills {},
        #[route("/articles")]
        Articles {},
        #[route("/projects")]
        Projects {},
        #[route("/contact")]
        Contact {},
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

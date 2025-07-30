use dioxus::prelude::*;
use crate::{BlogCategory, Route};

const default_navbar_elements: [(&str, &Route); 4] = [
    ("Home", &Route::Home {}),
    ("About", &Route::About {}),
    ("BQN", &Route::Category { category: BlogCategory::BQN }),
    ("Cantonese", &Route::Category { category: BlogCategory::Cantonese }),
];

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    let nav = navigator();
    let buttons = &default_navbar_elements;

    rsx! {
        div {
            id: "navbar",
            for &(name, route) in buttons.iter() {
                button { id: "navbar-button",
                    onclick: move |_| { nav.push(route.clone()); },
                    "{name}"
                }
            }
        }

        Outlet::<Route> {}
    }
}

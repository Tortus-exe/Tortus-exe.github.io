use dioxus::prelude::*;
use crate::{BlogCategory, Route};

const DEFAULT_NAVBAR_ELEMENTS: [(&str, &Route); 8] = [
    ("Home", &Route::Home {}),
    ("About", &Route::About {}),
    ("Skills", &Route::Skills {}),
    ("Articles", &Route::Articles {}),
    ("Projects", &Route::Projects {}),
    ("Contact", &Route::Contact {}),
    ("BQN", &Route::Category { category: BlogCategory::BQN }),
    ("Cantonese", &Route::Category { category: BlogCategory::Cantonese }),
];

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    let nav = navigator();
    let buttons = &DEFAULT_NAVBAR_ELEMENTS;

    rsx! {
        div { id: "navbar-parent",
            div { id: "navbar",
                div { id: "navbar-preview-element"
                    
                }
                for &(name, route) in buttons.iter() {
                    button { class: "navbar-button",
                        onclick: move |_| { nav.push(route.clone()); },
                        "{name}"
                    }
                }
            }
            div { id: "body-contents",
                Outlet::<Route> {}
            }
        }

    }
}

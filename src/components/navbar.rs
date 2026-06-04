use dioxus::prelude::*;
use crate::{BlogCategory, Route};

const DEFAULT_NAVBAR_ELEMENTS: [(&str, &Route); 6] = [
    ("Home", &Route::Home {}),
    ("About", &Route::About {}),
    ("Skills", &Route::Skills {}),
    ("Articles", &Route::Articles {}),
    ("Projects", &Route::Projects {}),
    ("Contact", &Route::Contact {}),
];

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    let nav = navigator();
    let buttons = &DEFAULT_NAVBAR_ELEMENTS;
    let mut hidden = use_signal(|| true);

    rsx! {
        div { class: "navbar-mobile-parent",
            div { class: "menu-bar-button",
                img { id: "menu-button-image",
                    onclick: move |_| {
                        hidden.toggle();
                    },
                    src: asset!("/assets/images/menu-button-mobile.png")
                }
            }
            div { id: "navbar-parent",
                div { class: {if hidden() {{"hide-on-mobile"}} else {{""}}}, id: "navbar",
                    div { class: "hide-on-mobile",
                        id: "navbar-preview-element",
                        img {
                            src: "https://avatars.githubusercontent.com/u/65506144?v=4"
                        }
                    }
                    for &(name, route) in buttons.iter() {
                        button { class: "navbar-button",
                            onclick: move |_| { 
                                hidden.toggle();
                                nav.push(route.clone()); 
                            },
                            "{name}"
                        }
                    }
                }
                div { class: {if hidden() {{""}} else {{"hide-on-mobile"}}} ,
                    id: "body-contents",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

use dioxus::prelude::*;
use std::fmt;
use crate::{BlogCategory, Route};

#[derive(Clone, PartialEq)]
pub enum ImageSource {
    S(String),
    A(Asset),
}

impl fmt::Display for ImageSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageSource::S(l) => l.fmt(f),
            ImageSource::A(l) => l.fmt(f)
        }
    }
}

const catImage: &str = "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRk6bAdCzXynKz3ycAf4L01YXR-vnGTxc3olQ&s";

#[component]
pub fn Articles() -> Element {
    let nav = navigator();

    rsx! {
        div { class: "category-list",
            id: "articles-list",
            CategoryPreview {
                source: ImageSource::S(catImage.to_string()), 
                name: "Cantonese",
                onclick: move |_| {nav.push(
                    Route::Category {category: BlogCategory::Cantonese});}
            }
            CategoryPreview {
                source: ImageSource::A(asset!("/assets/images/bqn.png")),
                name: "BQN",
                onclick: move |_| {nav.push(
                    Route::Category {category: BlogCategory::BQN});}
            }
        }
    }
}

#[component]
pub fn CategoryPreview(source: ImageSource, name: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "category-preview",
            onclick: onclick,
            img { class: "category-preview-img",
                src: "{source}"
            }
            p { class: "category-preview-title",
                "{name}"
            }
        }
    }
}

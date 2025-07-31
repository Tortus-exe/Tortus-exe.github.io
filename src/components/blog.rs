use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use crate::components::categories::{BlogCategory, GetBlogList};

/// use_asset_handler() -> LOOK INTO
/// THIS MAY NOT WORK IN 0.6.0. PLEASE CHECK.

/// Blog page
#[component]
pub fn Blog(category: BlogCategory, name: String) -> Element {
    let filename: Option<&'static str> = GetBlogList(&category)
                                    .into_iter()
                                    .find(|x| {x.title == name})
                                    .map(|x| x.asset);
    match filename {
        None => rsx! { h1 { "this page is not found!" } },
        Some(contents) => {
            rsx! {
                div { id: "blog",
                    Markdown {
                        src: contents
                    }
                }
            }
        }
    }
}

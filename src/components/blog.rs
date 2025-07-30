use dioxus::prelude::*;
use crate::components::categories::BlogCategory;

/// Blog page
#[component]
pub fn Blog(category: BlogCategory, name: String) -> Element {
    rsx! {
        div { id: "blog",
            h1 { "This is blog #{name}!" }
        }
    }
}

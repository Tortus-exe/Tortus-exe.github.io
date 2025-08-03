use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use crate::components::categories::{BlogCategory, get_blog_list};
use gloo_net::http::Request;

/// use_asset_handler() -> LOOK INTO
/// THIS MAY NOT WORK IN 0.6.0. PLEASE CHECK.

/// Blog page
#[component]
pub fn Blog(category: BlogCategory, name: String) -> Element {
    let filename: Option<String> = get_blog_list(&category)
                                    .iter()
                                    .find(|x| {x.title == name})
                                    .map(|x| x.asset.to_string());
    match filename {
        None => rsx! { h1 { "this page is not found!" } },
        Some(filename_success) => { blog_if_exists(filename_success) }
    }
}

fn blog_if_exists(filename: String) -> Element {
    let mut contents = use_signal(|| "".to_string());
    use_future(move || {
        let fnameclone = filename.clone();
        async move {
            let response = Request::get(&fnameclone)
                .send()
                .await.unwrap();
            if response.ok() {
                contents.set(response.text().await.unwrap())
            } else {
                contents.set(response.status_text());
            }
    }});

    rsx! {
        div { class: "blog",
            Markdown {
                src: "{contents}"
            }
        }
    }
}

use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use crate::utils::getAsset::getAsset;

#[component]
pub fn About() -> Element {
    let mut about_text = use_signal(|| "".to_string());
    use_future(move || async move {
        let about_text_name = asset!("/assets/about.md");
        let result = getAsset(about_text_name).await;
        match(result) {
            Ok(response) => about_text.set(response),
            Err(error) => about_text.set(error)
        }
    });

    rsx! {
        Markdown { src: "{about_text}" }
    }
}


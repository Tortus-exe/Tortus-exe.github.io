use dioxus::prelude::*;
use gloo_net::http::Request;
use dioxus_markdown::Markdown;

#[component]
pub fn About() -> Element {
    let mut about_text = use_signal(|| "".to_string());
    use_future(move || async move {
        let about_text_fut = Request::get(asset!("/assets/about.md").to_string().as_str()).send();
        let response = about_text_fut.await.unwrap();
        if response.ok() {
            about_text.set(response.text().await.unwrap());
        } else {
            about_text.set(response.status_text());
        }
    });

    rsx! {
        Markdown { src: "{about_text}" }
    }
}


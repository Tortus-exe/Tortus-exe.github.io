use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use crate::components::articles::ImageSource;
use crate::components::projects::{ProjectPageData, get_project_page_data, ProjectProperties, BackgroundInfo};
use gloo_net::http::Request;

#[component]
pub fn ProjectPage(title: String) -> Element {
    let data: Option<&ProjectPageData> = get_project_page_data(title);

    data.map_or(rsx! {
            div {
                "Sorry! There is no project with this name!"
            }
        },
        |d| rsx! {
        {d.background.clone().map(|bg| {
        rsx!{
            document::Style {
                r#"
                    body:has(.project) {{
                        background-color: {bg.color};
                    }}
                "#
            }
            {bg.symbol.map(|imgsrc| rsx! {
              img { class: "project-page-symbol-image",
                src: "{imgsrc}"
              }
            })}
        }})}
        ProjectPageTemplate {
            filename: d.filename.to_string(),
            icon: d.icon.clone(), 
            projprops: &d.projprops,
        }
    })
}

#[component]
fn ProjectPageTemplate(filename: String, icon: Option<ImageSource>, projprops: &'static ProjectProperties) -> Element {
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
        }
    });

    rsx! {
        div { class: "project",
            ProjectPropertyDisplay {
                projprops: projprops
            }
            {icon.map(|src| rsx! {
                img { class: "project-page-icon",
                    src: "{src}"
                }
            })}
            Markdown {
                src: "{contents}",
            }
        }
    }
}

#[component]
pub fn ProjectPropertyDisplay(projprops: &'static ProjectProperties) -> Element {
    rsx! {
        div { class: "project-properties",
            {projprops.language.map(|lang| rsx! {
                p { class: "proj-lang", "language: {lang}" } } ) }
            {projprops.link.map(|link| rsx! {
                p { class: "proj-link", "source: " 
                    a { href: "{link}", "{link}" }
                } } ) }
            {projprops.status.map(|stat| rsx! {
                p { class: "proj-status", "status: {stat}" } } ) }
            {projprops.collabs.map(|c| rsx! {
                p { class: "proj-collabs", "in collaboration with: {c}" } }
            )}
        }
    }
}
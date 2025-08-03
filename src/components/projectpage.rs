use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use crate::components::articles::ImageSource;
use crate::components::projects::{ProjectPageData, get_project_page_data, ProjectProperties};
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
        ProjectPageTemplate {
            filename: d.filename.to_string(),
            background: d.background.clone(), 
            icon: d.icon.clone(), 
            projprops: &d.projprops,
        }
    })
}

#[component]
fn ProjectPageTemplate(filename: String, background: Option<ImageSource>, icon: Option<ImageSource>, projprops: &'static ProjectProperties) -> Element {
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
    let urlString: String = background.clone().map_or("".to_string(), 
            |bg| { bg.to_string() });

    rsx! {
        div { class: "project",
            style: "background-image: url('{urlString}');",
            {icon.map(|src| rsx! {
                img { class: "project-page-icon",
                    src: "{src}"
                }
            })}
            Markdown {
                src: "{contents}",
            }
            ProjectPropertyDisplay {
                projprops: projprops
            }
        }
    }
}

#[component]
pub fn ProjectPropertyDisplay(projprops: &'static ProjectProperties) -> Element {
    rsx! {
        div { class: "project-properties",
            {projprops.language.map(|lang| rsx! {
                p { class: "proj-lang", "language: {lang}" }
            })}
        }
    }
}
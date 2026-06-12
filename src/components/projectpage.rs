use dioxus::prelude::*;
use comrak::{markdown_to_html, Options};
use crate::components::articles::ImageSource;
use crate::components::projects::{ProjectPageData, get_project_page_data, ProjectProperties, BackgroundInfo};
use crate::utils::getAsset::getAsset;
use crate::components::blog::REGISTERED_ASSETS;
use std::sync::Arc;
use dioxus::logger::tracing;

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
            filename: d.filename,
            icon: d.icon.clone(), 
            projprops: &d.projprops,
        }
    })
}

#[component]
fn ProjectPageTemplate(filename: Asset, icon: Option<ImageSource>, projprops: &'static ProjectProperties) -> Element {
    let mut contents = use_signal(|| "".to_string());
    use_future(move || {
        let fnameclone = filename.clone();
        async move {
            let result = getAsset(fnameclone).await;
            match result {
                Ok(response) => contents.set(response),
                Err(error) => contents.set(error)
            }
        }
    });

    let mut options = Options::default();
    options.extension.footnotes = true;
    options.extension.header_id_prefix = Some("sect-".to_string());
    options.render.r#unsafe = true;
    options.extension.image_url_rewriter = Some(Arc::new(
        |url: &str| {
            if url.starts_with('?') {
                let result = REGISTERED_ASSETS.get(&url[1..]);
                // tracing::debug!("{}: {:?}", url, result);
                match result {
                    Some(a) => a.to_string(),
                    None => url.to_string()
                }
            } else {
                url.to_string()
            }
        }
    ));

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
            div {
                dangerous_inner_html: markdown_to_html(&contents.read(), &options)
            }

            // Markdown {
            //     src: "{contents}",
            // }
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
use dioxus::prelude::*;
use dioxus_markdown::{Markdown, CustomComponents};
use crate::components::categories::{BlogCategory, get_blog_list};
use crate::utils::getAsset::getAsset;
// use markdown::{to_html_with_options, Options, CompileOptions};
use comrak::{markdown_to_html, Options};
use std::sync::Arc;
use phf::{Map, phf_map};

static REGISTERED_ASSETS: Map<&'static str, Asset> = phf_map! {
    "kawajapa_train" => asset!("/assets/images/A-B-train.png"),
    "kawajapa_sakura_ga_aruku" => asset!("/assets/images/A-B-train-sakura-ga-aruku.png"),
    "kawajapa_sakura_ga_nihonjin_da" => asset!("/assets/images/A-B-train-sakura-ga-nihonjin-da.png"),
    "kawajapa_sakura_da" => asset!("/assets/images/A-B-train-sakura-da.png"),
    "kawajapa_keeki" => asset!("/assets/images/A-B-train-keeki.png"),
    "kawajapa_keeki_zero" => asset!("/assets/images/A-B-train-keeki-zero.png"),
};

/// Blog page
#[component]
pub fn Blog(category: BlogCategory, name: String) -> Element {
    let filename: Option<Asset> = get_blog_list(&category)
                                    .iter()
                                    .find(|x| {x.title == name})
                                    .map(|x| x.asset);
    match filename {
        None => rsx! { h1 { "this page is not found!" } },
        Some(filename_success) => { blog_if_exists(filename_success) }
    }
}

#[component]
pub fn Color(style: String, text: String) -> Element {
    rsx! {
        span {
            style: style,
            "{text}"
        }
    }
}

fn blog_if_exists(filename: Asset) -> Element {
    let mut components = CustomComponents::new();
    components.register("Color", |props| {
        Ok(rsx! {
            Color { 
                style: props.get("style").unwrap(), 
                text: props.get("text").unwrap()
            }
        })
    });
    let mut contents = use_signal(|| "".to_string());
    use_future(move || {
        let fnameclone = filename.clone();
        async move {
            let result = getAsset(filename).await;
            match result {
                Ok(response) => contents.set(response),
                Err(error) => contents.set(error)
            }
    }});

    let mut options = Options::default();
    options.extension.footnotes = true;
    options.extension.header_id_prefix = Some("sect-".to_string());
    options.render.r#unsafe = true;
    options.extension.image_url_rewriter = Some(Arc::new(
        |url: &str| {
            if url.starts_with('?') {
                let result = REGISTERED_ASSETS.get(&url[1..]);
                match result {
                    Some(a) => a.to_string(),
                    None => url.to_string()
                }
            } else {
                url.to_string()
            }
        }
    ));

    // let options = Options {
    //     compile: CompileOptions {
    //         allow_dangerous_html: true, // CRITICAL: This lets <span style="..."> work!
    //         ..CompileOptions::default()
    //     },
    //     ..Options::default()
    // };

    rsx! {
        div { class: "blog",
            // dangerous_inner_html: to_html_with_options(&contents.read(), &options).unwrap()
            dangerous_inner_html: markdown_to_html(&contents.read(), &options)
            // Markdown {
            //     src: "{contents}",
            //     components
            // }
        }
    }
}

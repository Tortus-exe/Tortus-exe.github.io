use dioxus::prelude::*;
use crate::components::articles::ImageSource;
use crate::Route;

#[derive(Clone, PartialEq)]
pub struct ProjectProperties {
    pub language: Option<&'static str>,
    pub link: Option<&'static str>,
    pub status: Option<&'static str>,
    pub collabs: Option<&'static str>,
}

#[derive(Clone, PartialEq)]
enum ProjectRouting {
    Custom(Route),
    Template(&'static str),
}

impl ProjectRouting {
    pub fn to_route(&self) -> Route {
        match self {
            ProjectRouting::Custom(x) => x.clone(),
            ProjectRouting::Template(y) => Route::ProjectPage {title: y.to_string()}
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct BackgroundInfo {
    pub color: &'static str,
    pub symbol: Option<ImageSource>
}

#[derive(Clone, PartialEq)]
pub struct ProjectPageData {
    pub image: ImageSource,
    pub title: &'static str, 
    pub desc: &'static str, 
    pub icon: Option<ImageSource>,
    pub route: ProjectRouting,
    pub filename: Asset,
    pub background: Option<BackgroundInfo>,
    pub projprops: ProjectProperties
}

const PROJECTS: [ProjectPageData;1] = [
    ProjectPageData {
        image: ImageSource::A(asset!("/assets/images/stackVMBG.png")),
        title: "StackVM",
        desc: "A small stack machine written in C. Meant for lightweight applications, such as memory-limited computer chips.",
        icon: Some(ImageSource::A(asset!("/assets/images/stackVM.png"))),
        route: ProjectRouting::Template("StackVM"),
        filename: asset!("/assets/projects/stackvm.md"),
        background: Some(BackgroundInfo {
            color: "#1f0020",
            symbol: Some(ImageSource::A(asset!("/assets/images/stackVMBG.png")))
        }),
        projprops: ProjectProperties {
            language: Some("C"),
            link: Some("https://github.com/Tortus-exe/stackvm"),
            status: Some("completed"),
            collabs: None
        }
    }
];

pub fn get_project_page_data(title: String) -> Option<&'static ProjectPageData> {
    PROJECTS.iter().find(|x| {x.title == title})
}

#[component]
pub fn Projects() -> Element {
    let nav = navigator();

    rsx! {
        div { id: "project-grid",
            for p in PROJECTS.iter() {
                ProjectPreview {
                    image: p.image.clone(),
                    title: p.title,
                    desc: p.desc,
                    icon: p.icon.clone(),
                    onclick: move |_| {nav.push(
                        p.route.to_route());}
                }
            }
        }
    }
}

#[component]
pub fn ProjectPreview(image: ImageSource, title: String, desc: String, icon: Option<ImageSource>, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "project-preview",
            onclick: onclick,
            img { class: "project-preview-img",
                src: "{image}"
            }
            p { class: "project-preview-title",
                "{title}"
                {icon.map(|src| rsx! {
                    img { class: "project-icon",
                        src: "{src}"
                    }
                })}
            }
            p { class: "project-preview-description",
                "{desc}"
            }
        }
    }
}

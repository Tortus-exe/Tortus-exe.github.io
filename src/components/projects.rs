use dioxus::prelude::*;
use crate::components::articles::ImageSource;
use crate::Route;

#[derive(Clone, PartialEq)]
pub struct ProjectProperties {
    pub language: Option<&'static str>
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
pub struct ProjectPageData {
    pub image: ImageSource,
    pub title: &'static str, 
    pub desc: &'static str, 
    pub icon: Option<ImageSource>,
    pub route: ProjectRouting,
    pub filename: Asset,
    pub background: Option<ImageSource>,
    pub projprops: ProjectProperties
}

const PROJECTS: [ProjectPageData;1] = [
    ProjectPageData {
        image: ImageSource::A(asset!("/assets/images/bqn.png")),
        title: "First Project",
        desc: "This is my first project.",
        icon: None,
        route: ProjectRouting::Template("First Project"),
        filename: asset!("/assets/projects/firstproject.md"),
        background: None,
        projprops: ProjectProperties {
            language: Some("BQN"),
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

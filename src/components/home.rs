use dioxus::prelude::*;
use crate::{BlogCategory, Route};

struct Home_code {
    pre_about: &'static str,
    about: &'static str,
    pre_projects: &'static str,
    projects: &'static str,
    pre_skills: &'static str,
    skills: &'static str,
    pre_articles: &'static str,
    articles: &'static str,
    pre_contact: &'static str,
    contact: &'static str,
    tail: &'static str,
}

const hs_home: Home_code = Home_code {
    pre_about:
r#"type "#,
    about:
r#"About_Me"#,
    pre_projects:
r#" = String
type Learn = Maybe

"#,
    projects:
r#"projects"#,
    pre_skills:
r#" :: Learn About_Me -> IO ()
projects f = case f of
    Just "#,
    skills:
r#"skills"#,
    pre_articles:
r#" -> print $ skills
                <> " "#,
    articles:
r#"articles!"#,
    pre_contact:
r#""
    Nothing -> error ""#,
    contact:
r#"Contact?"#,
    tail:
r#""

main = projects $ Just "interesting"
"#
};

const c_home: Home_code = Home_code {
    pre_about:
r#"
#include ""#,
    about:
r#"about_me.h"#,
    pre_projects:
r#""
#define CONTACT_ME(a,b) projects(a,b)

void "#,
    projects:
r#"projects"#,
    pre_skills:
r#"(char* a, int n) {
    char "#,
    skills:
r#"skills"#,
    pre_articles:
r#"[] = "cool!";
    printf("my "#,
    articles:
r#"articles"#,
    pre_contact:
r#" %s\n", a);
    if(you.interested && n>0)
        "#,
    contact:
r#"CONTACT_ME"#,
    tail:
r#"(a, --n);
}

void main() { projects("are cool!", 5); }
"#,
};

const code: [Home_code;2] = [
    hs_home,
    c_home
];

#[component]
pub fn Home() -> Element {
    let text: usize = (getrandom::u32().unwrap_or(0) as usize) % code.len();
    let nav = navigator();

    rsx! {
        div { class: "homepage_header",
            h1 { "Tort's homepage" }
        }
        div { class: "homepage_code",
            p {class: "grayed_out", "{code[text].pre_about}"}
            button {onclick: move |_| {nav.push(Route::About {});}, class: "grayed_out", id: "about_home", "{code[text].about}"}
            p {class: "grayed_out", "{code[text].pre_projects}"}
            button {onclick: move |_| {nav.push(Route::Projects {});}, class: "grayed_out", id: "projects_home", "{code[text].projects}"}
            p {class: "grayed_out", "{code[text].pre_skills}"}
            button {onclick: move |_| {nav.push(Route::Skills {});}, class: "grayed_out", id: "skills_home", "{code[text].skills}"}
            p {class: "grayed_out", "{code[text].pre_articles}"}
            button {onclick: move |_| {nav.push(Route::Category {category: BlogCategory::Cantonese});}, class: "grayed_out", id: "articles_home", "{code[text].articles}"}
            p {class: "grayed_out", "{code[text].pre_contact}"}
            button {onclick: move |_| {nav.push(Route::Contact {});}, class: "grayed_out", id: "contact_home", "{code[text].contact}"}
            p {class: "grayed_out", "{code[text].tail}"}
        }
    }
}
use dioxus::prelude::*;
use std::str::FromStr;
use crate::Route;

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum BlogCategory {
    BQN,
    Cantonese
}

pub struct ParseCategoryError;

impl FromStr for BlogCategory {
    type Err = ParseCategoryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BQN" => Ok(BlogCategory::BQN),
            "Cantonese" => Ok(BlogCategory::Cantonese),
            _ => Err(ParseCategoryError)
        }
    }
}

impl ToString for BlogCategory {
    fn to_string(&self) -> String {
        match self {
            BlogCategory::BQN => "BQN",
            BlogCategory::Cantonese => "Cantonese",
        }.to_string()
    }
}

impl std::fmt::Display for ParseCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "Category parse error!");
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
struct BlogEntry {
    filename: &'static str,
    title: &'static str,
}

const bqn_categories: [BlogEntry; 1] = [
    BlogEntry {filename: "introduction.md", title: "introduction"},
];

const cantonese_categories: [BlogEntry; 6] = [
    BlogEntry {filename: "phonetics.md", title: "phonetics"},
    BlogEntry {filename: "sentences.md", title: "sentences"},
    BlogEntry {filename: "time_and_tense.md", title: "time and tense"},
    BlogEntry {filename: "questions_and_answers.md", title: "questions and answers"},
    BlogEntry {filename: "orders.md", title: "orders, orders!"},
    BlogEntry {filename: "numbers.md", title: "from one to infinity"},
];

fn GetBlogList(category: &BlogCategory) -> Box<[BlogEntry]> {
    match category {
        BlogCategory::BQN => Box::new(bqn_categories),
        BlogCategory::Cantonese => Box::new(cantonese_categories),
    }
}

#[component]
pub fn Category(category: BlogCategory) -> Element {
    let blog_list = GetBlogList(&category);

    rsx! {
        div { id: "category_listing",
            div {
                for (num, b) in blog_list.into_iter().enumerate() {
                    CategoryButton { 
                        blogCategory: category,
                        blogEntry: b.clone(), 
                        num: num 
                    }
                }
            }
            div {
                
            }
        }
    }
}

#[component]
pub fn CategoryButton(blogCategory: BlogCategory, blogEntry: BlogEntry, num: usize) -> Element {
    let nav = navigator();
    // let title = String::from(blogElem.title).clone();
    let title = blogEntry.title;

    rsx! {
        button { id: "category_button",
            onclick: move |_| { nav.push(Route::Blog { 
                category: blogCategory,
                name: title.to_string(),
            }); },
            "{num} - {title}"
        }
    }
}

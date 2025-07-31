use dioxus::prelude::*;
use std::str::FromStr;
use crate::Route;

const BQN_CATEGORIES: [BlogEntry; 1] = [
    BlogEntry {asset: crate::articles::bqn::introduction::contents, title: "introduction"},
];

const CANTONESE_CATEGORIES: [BlogEntry; 6] = [
    BlogEntry {asset: crate::articles::cantonese::phonetics::contents, title: "phonetics"},
    BlogEntry {asset: crate::articles::cantonese::sentences::contents, title: "sentences"},
    BlogEntry {asset: crate::articles::cantonese::time_and_tense::contents, title: "time and tense"},
    BlogEntry {asset: crate::articles::cantonese::questions_and_answers::contents, title: "questions and answers"},
    BlogEntry {asset: crate::articles::cantonese::orders::contents, title: "orders, orders!"},
    BlogEntry {asset: crate::articles::cantonese::numbers::contents, title: "from one to infinity"},
];

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
pub struct BlogEntry {
    pub asset: &'static str,
    pub title: &'static str,
}

pub fn GetBlogList(category: &BlogCategory) -> Box<[BlogEntry]> {
    match category {
        BlogCategory::BQN => Box::new(BQN_CATEGORIES),
        BlogCategory::Cantonese => Box::new(CANTONESE_CATEGORIES),
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
                        blog_category: category,
                        blog_entry: b.clone(), 
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
fn CategoryButton(blog_category: BlogCategory, blog_entry: BlogEntry, num: usize) -> Element {
    let nav = navigator();
    let title = blog_entry.title;

    rsx! {
        button { id: "category_button",
            onclick: move |_| { nav.push(Route::Blog { 
                category: blog_category,
                name: title.to_string(),
            }); },
            "{num} - {title}"
        }
    }
}

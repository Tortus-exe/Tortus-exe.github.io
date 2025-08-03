use dioxus::prelude::*;
use std::str::FromStr;
use crate::Route;
use crate::components::articles::{ CategoryPreview, ImageSource };

const BQN_CATEGORIES: [BlogEntry; 1] = [
    BlogEntry {asset: asset!("/assets/bqn/introduction.md"), title: "introduction", thumbnail: ImageSource::A(asset!("/assets/images/bqn.png"))},
];

const CANTONESE_CATEGORIES: [BlogEntry; 6] = [
    BlogEntry {asset: asset!("/assets/cantonese/phonetics.md"), title: "phonetics", thumbnail: ImageSource::A(asset!("/assets/images/cantonese.png"))},
    BlogEntry {asset: asset!("/assets/cantonese/sentences.md"), title: "sentences", thumbnail: ImageSource::A(asset!("/assets/images/cantonese.png"))},
    BlogEntry {asset: asset!("/assets/cantonese/time_and_tense.md"), title: "time and tense", thumbnail: ImageSource::A(asset!("/assets/images/cantonese.png"))},
    BlogEntry {asset: asset!("/assets/cantonese/questions_and_answers.md"), title: "questions and answers", thumbnail: ImageSource::A(asset!("/assets/images/cantonese.png"))},
    BlogEntry {asset: asset!("/assets/cantonese/orders.md"), title: "orders, orders!", thumbnail: ImageSource::A(asset!("/assets/images/cantonese.png"))},
    BlogEntry {asset: asset!("/assets/cantonese/numbers.md"), title: "from one to infinity", thumbnail: ImageSource::A(asset!("/assets/images/cantonese.png"))},
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
        write!(f, "Category parse error!")
    }
}

#[derive(Clone, PartialEq)]
pub struct BlogEntry {
    pub asset: Asset,
    pub title: &'static str,
    pub thumbnail: ImageSource,
}

pub fn get_blog_list(category: &BlogCategory) -> Box<[BlogEntry]> {
    match category {
        BlogCategory::BQN => Box::new(BQN_CATEGORIES),
        BlogCategory::Cantonese => Box::new(CANTONESE_CATEGORIES),
    }
}

#[component]
pub fn Category(category: BlogCategory) -> Element {
    let blog_list = get_blog_list(&category);

    rsx! {
        div { id: "category_listing",
            div {
                for (num, b) in blog_list.iter().enumerate() {
                    CategoryButton { 
                        blog_category: category,
                        blog_entry: b.clone(), 
                        num: num,
                    }
                }
            }
        }
    }
}

#[component]
fn CategoryButton(blog_category: BlogCategory, blog_entry: BlogEntry, num: usize) -> Element {
    let nav = navigator();
    let title = blog_entry.title;

    rsx! {
        CategoryPreview {
            onclick: move |_| { nav.push(Route::Blog { 
                category: blog_category,
                name: title.to_string(),
            }); },
            name: "{num} - {title}",
            source: blog_entry.thumbnail
        }
    }
}

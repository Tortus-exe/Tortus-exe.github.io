use dioxus::prelude::*;

const buttons: [(&str, Asset, &str); 3] = [
    ("mail", asset!("assets/images/mail.png"), "mailto:curtis.kc.chen"),
    ("linkedin", asset!("assets/images/linkedin.png"), "https://www.linkedin.com/in/curtis-chen-ubcece/"),
    ("github", asset!("assets/images/github.png"), "https://github.com/Tortus-exe/"),
];

#[component]
pub fn Contact() -> Element {
    rsx! {
        div { class: "contact_page", 
            id: "button_panel",
            for (id, src, link) in buttons.iter() {
                a { href: link.to_string(),
                    target: "_blank",
                    img { class: "contact_page",
                        id: id.to_string(),
                        src: *src
                    }
                }
            }
        }
        div { class: "contact_page",
            id: "descriptions",
            p { "mail: curtis.kc.chen@gmail.com" }
            p { "Curtis Chen" }
            p { "UBC ECE New-Graduate Student" }
        }
    }
}

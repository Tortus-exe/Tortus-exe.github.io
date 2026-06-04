use dioxus::prelude::*;

const SKILLS: [(u16, &str, &str); 11] = [
    (100, "#FFD43B", "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/python/python-original.svg"), // python
    (100, "#5C6BC0", "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/c/c-original.svg"), // C
    (100, "#ED2B00", "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/java/java-original.svg"), // Java
    (95, "#00549D", "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/cplusplus/cplusplus-original.svg"),
    (92, "#D34516", "https://github.com/rust-lang/rust-artwork/raw/refs/heads/master/logo/rust-logo-white-outline.svg"),
    (92, "#00007D", "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/lua/lua-original.svg"),
    (90, "#00ae30", "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/apl/apl-original.svg"),
    (86, "#FFD600", "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/javascript/javascript-original.svg"),
    (80, "#26655d", "https://www.aplwiki.com/images/thumb/4/4d/BQN_logo.png/600px-BQN_logo.png?20201228230022"),
    (40, "#ffffff", "https://upload.wikimedia.org/wikipedia/commons/thumb/4/48/Lisp_logo.svg/960px-Lisp_logo.svg.png?_=20201113170541"),
    (30, "#7E57C2", "https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/julia/julia-original-wordmark.svg"),
];

#[component]
pub fn Skills() -> Element {
    rsx! {
        div { class: "skills_list",
            for (p, c, t) in SKILLS.iter() {
                RoundProgressBar { progress: *p, color: c.to_string(), href: t.to_string() }
            }
        }
        // ProgressBar { progress: 98, color: "#4000ec", tag: "progress" }
    }
}

#[component]
pub fn ProgressBar(progress: u16, color: String, tag: String) -> Element {
    rsx! {
        div { class: "progress_bar",
            div { class: "progress_bar_inside",
                style: "width:{progress}%;background-color:{color}",
                div {
                    "{tag}"
                }
            }
        }
    }
}

#[component]
pub fn RoundProgressBar(progress: u16, color: String, href: String) -> Element {
    let value: u16 = 400-4*progress;
    rsx! {
        svg {
            width: 150,
            height: 150,
            image { class: "preview_image", 
                width: "60%", height: "60%",
                x: "50%", y: "50%",
                href: href
            }
            circle { class: "border_track", cx: "-75", cy: "75", r: "65",
                style: "stroke-dashoffset:{value};stroke:{color}"
            }
        }
    }
}

use dioxus::prelude::*;

pub fn About() -> Element {
    rsx! {
        h1 { "Hello!" }
        p { "For those who know me well, my name is Tortus. I am a programmer with special interest in many adjacent fields, such as linguistics and math." br {} br {}

"I daily drive APL and BQN, the twin array programming languages, mostly for their aptitude in quick problem solving and terseness. I also daily drive Haskell and C for my compiled languages, usually reserved for large or compiled projects. My programming language exploration goes FAR beyond those. Friends know me well for my knowledge of Common Lisp, C++, Lua, Julia, Java, JS (thanks, react!), and pretty much everything one can throw at me. APL remains one of my favourites to this day, however. Who knew that the best ideas were born so long ago, and surpassed by few since then?" br {} br {}

"I speak English, Cantonese, and Japanese. I am a native English speaker (as you can hopefully tell!), a heritage speaker of Cantonese, and only an intermediate speaker of Japanese. As my studies in languages went forward, I began to accrue a greater appreciation of language in general. I now hope to use this website to make it easier for people who can speak English to learn Cantonese. Long live the freedom of Hong Kong!" br {} br {}

"Feel free to peruse the projects on my "
        a { href: "https://www.github.com/tortus-exe", "github" } 
        "." br {} br {}

"Oh yeah... right. The website. This website used to be written in JSX and React, but then I discovered " a { href: "https://dioxuslabs.com/", "Dioxus" } ". Boy... Dioxus just feels a bit like a better, faster, typechecked React to me so far, so I'm hoping it will revolutionize my website development. Apologies to those who were hoping to see blogs and projects; they will be up soon!" br {} br {}

"       See you on the flipside!    - Tort"
        }
    }
}


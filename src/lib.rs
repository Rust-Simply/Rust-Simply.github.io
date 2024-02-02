#![allow(non_snake_case)]

use dioxus::prelude::*;
use indoc::indoc;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Title {}
        Intro {}
        Iriss {}
        Footer {}
    })
}

fn Title(cx: Scope) -> Element {
    render! {
        header {
            id: "title",
            img {
                src: "phio.svg"
            }
            h1 {
                "Rust, simply"
            }
        }
    }
}

fn Intro(cx: Scope) -> Element {
    render! {
        article {
            id: "intro",

            blockquote {
                p {
                    indoc! { r#"
                        "I often hear that Rust is a hard language to learn... but I don't think it is. I want to show
                        people that, yes, like any language it has its quirks, but today Rust is as easy if not easier
                        to pick up than many of its contemporaries"
                    "# }
                }
                cite { span {"Daniel"} " " span {"Mason"}}
            }
        }
    }
}

fn Iriss(cx: Scope) -> Element {
    render! {
        article {
            id: "iriss",
            h2 {
                span { "Idiomatic" }" "
                span { "Rust" }" "
                span { "in" }" "
                span { "Simple" }" "
                span { "Steps" }
            }
            section {
                p {
                    indoc! { "
                        Idiomatic Rust in Simple Steps (IRISS) is a step-by-step guide to learning Rust no programming
                        knowledge required. By focusing on idioms, the series aims to get you productive as quickly as possible.
                    " }
                }
                p {
                    indoc! { "
                        There are two parts to IRISS, a YouTube Series that's easy to watch, and a written guide that
                        provides more details.
                    " }
                }
            }
            section {
                class: "fifty-fifty",
                a {
                    href: "https://www.youtube.com/playlist?list=PLW2L8KbM0O7aRi_Bt4YE1JuW9EdMs0ztR",
                    img {
                        src: "./images/iriss-youtube.jpeg"
                    }
                    figcaption { "YouTube" }
                }
                a {
                    href: "/idiomatic-rust-in-simple-steps",
                    img {
                        src: "./images/iriss-book.png"
                    }
                    figcaption {
                        "The book"
                    }
                }
            }
        }
    }
}

fn Footer(cx: Scope) -> Element {
    render! {
        footer {
            a {
                href: "https://github.com/Rust-Simply",
                img {
                    src: "./logos/github-mark.svg",
                    alt: "GitHub"
                }
            }
            a {
                href: "https://www.youtube.com/@rustsimply",
                img {
                    src: "./logos/yt_icon_rgb.png",
                    alt: "YouTube"
                }
            }
        }
    }
}

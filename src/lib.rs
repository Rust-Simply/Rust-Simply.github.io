#![allow(non_snake_case)]

use dioxus::prelude::*;
use indoc::indoc;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Title {}
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

fn Iriss(cx: Scope) -> Element {
    render! {
        article {
            id: "iriss",
            h2 {
                "Idiomatic Rust in Simple Steps"
            }
            section {
                p {
                    indoc! { "
                        Idiomatic Rust in Simple Steps is a step-by-step guide to learning Rust no programming knowledge
                        required. By focusing on idioms, the series aims to get you productive as quickly as possible.
                    " }
                }
            }
            section {
                div {
                    "YouTube"
                }
                div {
                    "Book"
                }
            }
        }
    }
}

fn Footer(cx: Scope) -> Element {
    render! {
        footer {
            "GitHub"
        }
    }
}

fn main() {
    dioxus_desktop::launch(app);
}

use core::{fmt, str::FromStr};
use std::fmt::Display;

use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    let formatting = "formatting!";
    let formatting_tuple = ("a", "b");
    let lazy_fmt = format_args!("lazily formatted text");
    cx.render(rsx! {
        div {
            // Elements
            div {}
            h1 {"Some text"}
            h1 {"Some text with {formatting}"}
            h1 {"Formatting basic expressions {formatting_tuple.0} and {formatting_tuple.1}"}
            h1 {"Formatting without interpolation " formatting_tuple.0 "and" formatting_tuple.1 }
            h2 {
                "Multiple"
                "Text"
                "Blocks"
                "Use comments as separators in html"
            }

            div {
                class: "my special div",
                h1 {"Headers and attributes"}
            }

            div {
                class: lazy_fmt,
                id: format_args!("attributes can be passed lazily with std::fmt::Arguments"),
                div {
                    class: {
                        const WORD: &str = "expressions";
                        format_args!("Arguments can be passed in through curly braces for complex {WORD}")
                    }
                }
            }

            // Expressions can be used in element position too:
            rsx!( p { "More templating!"}),

            // Iterators
            (0..10).map(|i| rsx!(li {" {i} "})),
        }
    })
}

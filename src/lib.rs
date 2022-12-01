// https://github.com/rust-lang/rust/issues/104876
use perseus::{Html, PerseusApp, Template};
use sycamore::view;

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(|| {
        Template::new("index").template(|_| {
            view! {
                p { "Hello World!" }
            }
        })
    })
}

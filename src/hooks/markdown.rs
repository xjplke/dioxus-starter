use dioxus::prelude::use_hook;
use pulldown_cmark::{html, Options, Parser};

pub fn use_markdown() -> impl Fn(String) -> String {
    let options = Options::all();
    use_hook(|| {
        move |content: String| {
            let str = &content;
            let parser = Parser::new_ext(str, options);

            let mut output = String::new();
            html::push_html(&mut output, parser);

            output
        }
    })
}

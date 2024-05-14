use dioxus::prelude::*;

use crate::hooks::markdown::use_markdown;

#[component]
pub fn Href(to: String, children: Element) -> Element {
    rsx! {
        a { class: "text-cyan-700 dark:text-cyan-100 underline", href: "{to}", target: "_blank", {children} }
    }
}

// #[derive(Props)]
// pub struct MarkdownProps<'a> {
//     #[props(default)]
//     class: &'a str,
//     content: String,
// }

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default)]
    class: String,
    content: String,
}

pub fn Markdown<'a>(props: MarkdownProps) -> Element {
    let md_parser = use_markdown();
    let content = md_parser(props.content.clone());
    let extra_class = &props.class;
    rsx! {div { id: "markdown-body", class: "prose {extra_class}", dangerous_inner_html: "{content}" }}
}

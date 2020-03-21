extern crate pulldown_cmark;

use pulldown_cmark::{html, Parser};

pub fn md_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);

    let mut html_output: String = String::with_capacity(markdown.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    html_output
}

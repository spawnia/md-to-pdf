#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod md_to_html;

use core::fmt;
use md_to_html::md_to_html;
use rocket::request::Form;
use rocket::response::{content, NamedFile};
use std::fmt::{Display, Formatter};
use std::io::{Error, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

#[get("/")]
fn index() -> content::Html<String> {
    return content::Html(md_to_html(
        "
# md-to-pdf

An API service for converting markdown to PDF

## Routes

### POST /

Accepts markdown and responds with the converted PDF.

Send a form parameter `markdown` with the content to convert:

    curl -X POST -d 'markdown=# Heading 1' localhost:8000

You can also style the markdown through css:

    curl -X POST -d 'markdown=# Heading 1' -d 'css=h1 { color: red; }' localhost:8000

Depending on what features you prefer and the output that works best, you can
choose between two pdf conversion engines: `wkhtmltopdf` and `weasyprint`:

    curl -X POST -d 'markdown=# Heading 1' -d 'engine=weasyprint' localhost:8000

",
    ));
}

#[derive(FromFormValue)]
enum PdfEngine {
    Weasyprint,
    Wkhtmltopdf,
}

impl Display for PdfEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            PdfEngine::Weasyprint => write!(f, "weasyprint"),
            PdfEngine::Wkhtmltopdf => write!(f, "wkhtmltopdf"),
        }
    }
}

#[derive(FromForm)]
struct ConvertForm {
    markdown: String,
    css: Option<String>,
    engine: Option<PdfEngine>,
}

#[post("/", data = "<convert>")]
fn pandoc(convert: Form<ConvertForm>) -> Result<NamedFile, Error> {
    let mut pandoc_builder = Command::new("pandoc");

    let stdin = Stdio::piped();
    pandoc_builder.stdin(stdin);

    pandoc_builder.arg("--output=/tmp/markdown.pdf");

    let engine = convert.engine.as_ref().unwrap_or(&PdfEngine::Weasyprint);
    pandoc_builder.arg("--pdf-engine=".to_owned() + engine.to_string().as_str());

    let mut css_file;
    if convert.css.is_some() {
        css_file = NamedTempFile::new()?;
        css_file.write_all(convert.css.as_ref().unwrap().as_bytes())?;
        pandoc_builder.arg("--css=".to_owned() + css_file.path().to_str().unwrap());
    }

    let mut pandoc_process = pandoc_builder.spawn()?;

    {
        let pandoc_stdin = pandoc_process.stdin.as_mut().unwrap();
        pandoc_stdin.write_all(convert.markdown.as_bytes())?;
    }

    let output = pandoc_process.wait_with_output()?;
    debug!("{:?}", output);

    NamedFile::open(Path::new("/tmp/markdown.pdf"))
}

fn main() {
    // Heroku compatibility
    let port_string = std::env::var("PORT");
    match port_string {
        Ok(p) => std::env::set_var("ROCKET_PORT", p),
        Err(_e) => (),
    }

    rocket::ignite().mount("/", routes![index, pandoc]).launch();
}

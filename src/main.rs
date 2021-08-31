#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

use core::fmt;
use rocket::request::Form;
use rocket::response::{Debug, NamedFile};
use rocket_contrib::serve::StaticFiles;
use std::fmt::{Display, Formatter};
use std::io::{Error, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::Builder;

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
fn pandoc(convert: Form<ConvertForm>) -> Result<NamedFile, Debug<Error>> {
    let mut pandoc_builder = Command::new("pandoc");

    // Pandoc can not perform PDF conversion to STDOUT, so we need a temp file
    let pdf_temp_path = Builder::new()
        // Setting that suffix accomplishes two things:
        // - Pandoc will know that it should convert to PDF
        // - Rocket will set the correct Content-Type response header
        .suffix(".pdf")
        .tempfile()?
        .into_temp_path();
    let pdf_path = pdf_temp_path
        .to_str()
        .expect("Can not deal with non UTF-8 path.");
    pandoc_builder.arg("--output=".to_owned() + pdf_path);

    pandoc_builder.arg(
        "--pdf-engine=".to_owned()
            + convert
                .engine
                .as_ref()
                .unwrap_or(&PdfEngine::Weasyprint)
                .to_string()
                .as_str(),
    );

    // Declare outside of the if block to keep the file around until the end of this function if needed
    let mut css_file;
    if convert.css.is_some() {
        css_file = Builder::new()
            // Necessary for weasyprint to recognize it as a proper stylesheet
            .suffix(".css")
            .tempfile()?;
        css_file.write_all(convert.css.as_ref().unwrap().as_bytes())?;
        pandoc_builder.arg("--css=".to_owned() + css_file.path().to_str().unwrap());
    }

    // We can avoid writing the input to a file by streaming it to STDIN
    let stdin = Stdio::piped();
    pandoc_builder.stdin(stdin);

    let mut pandoc_process = pandoc_builder.spawn()?;

    {
        let pandoc_stdin = pandoc_process.stdin.as_mut().unwrap();
        pandoc_stdin.write_all(convert.markdown.as_bytes())?;
    }

    let output = pandoc_process.wait_with_output()?;
    debug!("{:?}", output);

    NamedFile::open(Path::new(pdf_path)).map_err(Debug)
}

fn main() {
    // Heroku compatibility
    match std::env::var("PORT") {
        Ok(p) => std::env::set_var("ROCKET_PORT", p),
        Err(_e) => (),
    }

    rocket::ignite()
        .mount("/", routes![pandoc])
        .mount("/", StaticFiles::from("static"))
        .launch();
}

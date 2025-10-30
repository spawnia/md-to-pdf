#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

use core::fmt;
use rocket::form::Form;
use rocket::fs::{FileServer, NamedFile};
use rocket::http::{ContentType, Method, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::fmt::{Display, Formatter};
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use tempfile::Builder;

#[derive(FromFormField)]
enum PdfEngine {
    Weasyprint,
    Wkhtmltopdf,
    Pdflatex,
}

impl Display for PdfEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            PdfEngine::Weasyprint => write!(f, "weasyprint"),
            PdfEngine::Wkhtmltopdf => write!(f, "wkhtmltopdf"),
            PdfEngine::Pdflatex => write!(f, "pdflatex"),
        }
    }
}

#[derive(FromForm)]
struct ConvertForm {
    markdown: String,
    css: Option<String>,
    engine: Option<PdfEngine>,
}

#[derive(Debug)]
enum ConvertError {
    Output(Output),
    IO(io::Error),
}

impl From<io::Error> for ConvertError {
    fn from(err: io::Error) -> ConvertError {
        ConvertError::IO(err)
    }
}

impl<'r> Responder<'r, 'static> for ConvertError {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        let mut builder = Response::build();
        match self {
            ConvertError::Output(output) => builder
                .header(ContentType::Plain)
                .sized_body(output.stderr.len(), io::Cursor::new(output.stderr))
                .status(Status::BadRequest),
            ConvertError::IO(err) => {
                error!("IO error during conversion: {}", err);
                builder.status(Status::InternalServerError)
            }
        };

        builder.ok()
    }
}

#[post("/", data = "<form>")]
async fn convert(form: Form<ConvertForm>) -> Result<NamedFile, ConvertError> {
    let mut pandoc_builder = Command::new("pandoc");

    // Pandoc can not perform PDF conversion to STDOUT, so we need a temp file
    let pdf_temp_path = Builder::new()
        // Setting that suffix accomplishes two things:
        // - Pandoc will know that it should convert to PDF
        // - Rocket will set the correct Content-Type response header
        .suffix(".pdf")
        .tempfile()
        .map_err(ConvertError::IO)?
        .into_temp_path();
    let pdf_path = pdf_temp_path
        .to_str()
        .expect("Can not deal with non UTF-8 path.");
    pandoc_builder.arg("--output=".to_owned() + pdf_path);

    pandoc_builder.arg(
        "--pdf-engine=".to_owned()
            + form
                .engine
                .as_ref()
                .unwrap_or(&PdfEngine::Weasyprint)
                .to_string()
                .as_str(),
    );

    // Declare outside of the if block to keep the file around until the end of this function if needed
    let mut css_file;
    if form.css.is_some() {
        css_file = Builder::new()
            // Necessary for weasyprint to recognize it as a proper stylesheet
            .suffix(".css")
            .tempfile()
            .map_err(ConvertError::IO)?;
        css_file
            .write_all(form.css.as_ref().unwrap().as_bytes())
            .map_err(ConvertError::IO)?;
        pandoc_builder.arg("--css=".to_owned() + css_file.path().to_str().unwrap());
    }

    // We can avoid writing the input to a file by streaming it to STDIN
    let stdin = Stdio::piped();
    pandoc_builder.stdin(stdin);

    pandoc_builder.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut pandoc_process = pandoc_builder.spawn().map_err(ConvertError::IO)?;

    pandoc_process
        .stdin
        .as_mut()
        .unwrap()
        .write_all(form.markdown.as_bytes())
        .map_err(ConvertError::IO)?;

    let output = pandoc_process
        .wait_with_output()
        .map_err(ConvertError::IO)?;
    debug!("{:?}", output);

    if !output.status.success() {
        return Err(ConvertError::Output(output));
    }

    NamedFile::open(Path::new(pdf_path))
        .await
        .map_err(ConvertError::IO)
}

#[launch]
fn rocket() -> _ {
    // https://github.com/lawliet89/rocket_cors/blob/v0.5.2/examples/fairing.rs
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Options]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .to_cors();

    // PDF conversion can take longer, so we increase timeouts for graceful shutdown.
    // - grace: 30s for ongoing PDF conversions to complete
    // - mercy: 10s for connection-level shutdown
    // Rocket defaults: grace=2s, mercy=3s (total 5s).
    // See: https://api.rocket.rs/v0.5/rocket/config/struct.Shutdown
    let shutdown_config = rocket::config::Shutdown {
        ctrlc: true,
        signals: std::collections::HashSet::from([rocket::config::Sig::Term]),
        grace: 30,
        mercy: 10,
        ..Default::default()
    };

    rocket::build()
        .configure(rocket::Config {
            shutdown: shutdown_config,
            ..Default::default()
        })
        .attach(cors.unwrap())
        .mount("/", routes![convert])
        .mount("/", FileServer::from("static"))
}

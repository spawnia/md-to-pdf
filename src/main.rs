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
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use tempfile::Builder;
use env_logger;

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
    header_template: Option<String>,
    footer_template: Option<String>,
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
            ConvertError::IO(_) => builder.status(Status::InternalServerError),
        };

        builder.ok()
    }
}

// ... existing code ...

#[post("/", data = "<form>")]
async fn convert(form: Form<ConvertForm>) -> Result<NamedFile, ConvertError> {
    let mut pandoc_builder = Command::new("pandoc");

    // Pandoc can not perform PDF conversion to STDOUT, so we need a temp file
    let pdf_temp_path = Builder::new()
        .suffix(".pdf")
        .tempfile()
        .map_err(|e| {
            error!("Failed to create temporary PDF file: {}", e);
            ConvertError::IO(e)
        })?
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

    // Handle CSS
    if let Some(css) = &form.css {
        let mut css_file = Builder::new()
            .suffix(".css")
            .tempfile()
            .map_err(|e| {
                error!("Failed to create temporary CSS file: {}", e);
                ConvertError::IO(e)
            })?;
        css_file
            .write_all(css.as_bytes())
            .map_err(|e| {
                error!("Failed to write to temporary CSS file: {}", e);
                ConvertError::IO(e)
            })?;
        pandoc_builder.arg("--css=".to_owned() + css_file.path().to_str().unwrap());
    }

    use std::env;

// ... existing code ...

// Handle header template
if let Some(header_template) = &form.header_template {
    if !header_template.is_empty() {
        let current_dir = env::current_dir().unwrap();
        let header_path = current_dir.join(format!("templates/{}", header_template));
        let header_path = header_path.canonicalize().unwrap(); // Utilisation du chemin absolu
        if !header_path.exists() {
            return Err(ConvertError::IO(io::Error::new(
                io::ErrorKind::NotFound,
                "Header template file not found",
            )));
        }
        let header_content = fs::read_to_string(&header_path).map_err(|e| {
            ConvertError::IO(e)
        })?;
        let mut header_file = Builder::new()
            .suffix(".html")
            .tempfile()
            .map_err(|e| {
                ConvertError::IO(e)
            })?;
        header_file
            .write_all(header_content.as_bytes())
            .map_err(|e| {
                ConvertError::IO(e)
            })?;
        let header_file_path = header_file.into_temp_path();
        let header_file_path_str = header_file_path.to_str().unwrap();
        pandoc_builder.arg("--include-in-header=".to_owned() + header_file_path_str);
    }
}

// Handle footer template
if let Some(footer_template) = &form.footer_template {
    if !footer_template.is_empty() {
        let current_dir = env::current_dir().unwrap();
        let footer_path = current_dir.join(format!("templates/{}", footer_template));
        let footer_path = footer_path.canonicalize().unwrap(); // Utilisation du chemin absolu
        if !footer_path.exists() {
            return Err(ConvertError::IO(io::Error::new(
                io::ErrorKind::NotFound,
                "Footer template file not found",
            )));
        }
        let footer_content = fs::read_to_string(&footer_path).map_err(|e| {
            ConvertError::IO(e)
        })?;
        let mut footer_file = Builder::new()
            .suffix(".html")
            .tempfile()
            .map_err(|e| {
                ConvertError::IO(e)
            })?;
        footer_file
            .write_all(footer_content.as_bytes())
            .map_err(|e| {
                ConvertError::IO(e)
            })?;
        let footer_file_path = footer_file.into_temp_path();
        let footer_file_path_str = footer_file_path.to_str().unwrap();
        pandoc_builder.arg("--include-after-body=".to_owned() + footer_file_path_str);
    }
}

    let stdin = Stdio::piped();
    pandoc_builder.stdin(stdin);

    pandoc_builder.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut pandoc_process = pandoc_builder.spawn().map_err(|e| {
        error!("Failed to spawn pandoc process: {}", e);
        ConvertError::IO(e)
    })?;

    pandoc_process
        .stdin
        .as_mut()
        .unwrap()
        .write_all(form.markdown.as_bytes())
        .map_err(|e| {
            error!("Failed to write to pandoc stdin: {}", e);
            ConvertError::IO(e)
        })?;

    let output = pandoc_process
        .wait_with_output()
        .map_err(|e| {
            error!("Failed to wait for pandoc process: {}", e);
            ConvertError::IO(e)
        })?;
    debug!("{:?}", output);

    if !output.status.success() {
        error!("Pandoc process failed with output: {:?}", output);
        return Err(ConvertError::Output(output));
    }

    NamedFile::open(Path::new(pdf_path))
        .await
        .map_err(|e| {
            error!("Failed to open generated PDF file: {}", e);
            ConvertError::IO(e)
        })
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    // https://github.com/lawliet89/rocket_cors/blob/v0.5.2/examples/fairing.rs
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Options]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("Error creating CORS fairing");

    rocket::build()
        .attach(cors)
        .mount("/", routes![convert])
        .mount("/static", FileServer::from("static"))
}
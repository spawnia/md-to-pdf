#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

use core::fmt;
use env_logger;
use rocket::form::Form;
use rocket::fs::{FileServer, NamedFile};
use rocket::http::{ContentType, Method, Status, Header};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::{json::Json, Serialize}; // <-- Needed for JSON
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{self, Write};
use std::path::{Path};
use std::process::{Command, Output, Stdio};
use tempfile::Builder;
use rocket::Either;

// ------------ PDF Engine enum ------------

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

// ------------ Form Data ------------

#[derive(FromForm)]
struct ConvertForm {
    markdown: String,
    css: Option<String>,
    engine: Option<PdfEngine>,
    header_template: Option<String>,
    footer_template: Option<String>,

    // NEW: Optional fields for saving the file permanently
    client_id: Option<String>,
    pdf_name: Option<String>,
}

// ------------ Error Handling ------------

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

// ------------ JSON response when we do have clientId/pdfName ------------

#[derive(Serialize)]
struct ConvertResponse {
    /// The download link to retrieve the PDF
    download_url: String,
}

// Add this struct and implementation
struct PdfResponse(Response<'static>);

impl<'r> Responder<'r, 'static> for PdfResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Ok(self.0)
    }
}

// ------------ PDF Generation Endpoint ------------

#[post("/", data = "<form>")]
async fn convert(form: Form<ConvertForm>) -> Result<Either<NamedFile, Json<ConvertResponse>>, ConvertError> {
    let form_data = form.into_inner();

    // Start building pandoc command
    let mut pandoc_builder = Command::new("pandoc");

    // Create a temporary PDF
    let pdf_temp_path = Builder::new()
        .suffix(".pdf")
        .tempfile()?;
    let pdf_path = pdf_temp_path
        .path()
        .to_str()
        .expect("Non UTF-8 path not supported");
    pandoc_builder.arg(format!("--output={}", pdf_path));

    // Choose the PDF engine, default to Weasyprint
    let engine = form_data.engine.unwrap_or(PdfEngine::Weasyprint);
    pandoc_builder.arg(format!("--pdf-engine={}", engine));

    // Handle CSS
    if let Some(css) = form_data.css {
        // Combine default and custom
        let default_css_path = "templates/default.css";
        let default_css = fs::read_to_string(default_css_path)?;
        let combined_css = format!("{}{}", default_css, css);

        let mut css_file = Builder::new().suffix(".css").tempfile()?;
        css_file.write_all(combined_css.as_bytes())?;
        let css_file_path = css_file.into_temp_path();
        let css_file_path_str = css_file_path.to_str().unwrap();
        pandoc_builder.arg(format!("--css={}", css_file_path_str));
    } else {
        // Use a default css if none given
        let default_css_path = "static/default_fonts.css";
        pandoc_builder.arg(format!("--css={}", default_css_path));
    }

    use std::env;

    // Handle header template
    if let Some(header_template) = form_data.header_template {
        if !header_template.is_empty() {
            let current_dir = env::current_dir().unwrap();
            let header_path = current_dir.join(format!("templates/{}", header_template));
            let header_path = header_path.canonicalize()?;
            if !header_path.exists() {
                error!("Header template file not found at: {:?}", header_path);
                return Err(ConvertError::IO(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Header template file not found",
                )));
            }
            let header_content = fs::read_to_string(&header_path)?;
            let mut header_file = Builder::new().suffix(".html").tempfile()?;
            header_file.write_all(header_content.as_bytes())?;
            let header_file_path = header_file.into_temp_path();
            let header_file_path_str = header_file_path.to_str().unwrap();
            pandoc_builder.arg(format!("--include-in-header={}", header_file_path_str));
        }
    }

    // Handle footer template
    if let Some(footer_template) = form_data.footer_template {
        if !footer_template.is_empty() {
            let current_dir = env::current_dir().unwrap();
            let footer_path = current_dir.join(format!("templates/{}", footer_template));
            let footer_path = footer_path.canonicalize()?;
            if !footer_path.exists() {
                error!("Footer template file not found at: {:?}", footer_path);
                return Err(ConvertError::IO(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Footer template file not found",
                )));
            }
            let footer_content = fs::read_to_string(&footer_path)?;
            let mut footer_file = Builder::new().suffix(".html").tempfile()?;
            footer_file.write_all(footer_content.as_bytes())?;
            let footer_file_path = footer_file.into_temp_path();
            let footer_file_path_str = footer_file_path.to_str().unwrap();
            pandoc_builder.arg(format!("--include-after-body={}", footer_file_path_str));
        }
    }

    pandoc_builder.stdin(Stdio::piped());
    pandoc_builder.stdout(Stdio::piped()).stderr(Stdio::piped());

    // Spawn pandoc
    let mut pandoc_process = pandoc_builder.spawn()?;
    // Write the markdown to pandoc
    pandoc_process
        .stdin
        .as_mut()
        .unwrap()
        .write_all(form_data.markdown.as_bytes())?;
    let output = pandoc_process.wait_with_output()?;

    if !output.status.success() {
        error!("Pandoc process failed with output: {:?}", output);
        return Err(ConvertError::Output(output));
    }

    // If we got clientId/pdfName, store the PDF and return JSON link
    if let (Some(client_id), Some(pdf_name)) = (form_data.client_id, form_data.pdf_name) {
        // Make sure the directory exists
        let client_dir = Path::new("public").join("pdf").join(&client_id);
        fs::create_dir_all(&client_dir)?;

        // Ensure pdf_name has .pdf extension
        let final_pdf_name = if !pdf_name.ends_with(".pdf") {
            format!("{}.pdf", pdf_name)
        } else {
            pdf_name
        };

        // Copy from temp to final location
        let out_path = client_dir.join(&final_pdf_name);
        fs::copy(pdf_path, &out_path)?;

        // Return a JSON with the link to download
        let download_link = format!("/download/{}/{}", client_id, final_pdf_name);
        Ok(Either::Right(Json(ConvertResponse { download_url: download_link })))
    } else {
        // Return the PDF file directly - fixed version
        Ok(Either::Left(NamedFile::open(pdf_temp_path.path()).await?))
    }
}

// ------------ Download Endpoint ------------

#[get("/<client_id>/<pdf_name>")]
async fn download_pdf(client_id: &str, pdf_name: &str) -> Option<PdfResponse> {
    let path = Path::new("public")
        .join("pdf")
        .join(client_id)
        .join(pdf_name);

    NamedFile::open(path).await.ok().map(|file| {
        // Ensure the filename has .pdf extension
        let download_name = if !pdf_name.ends_with(".pdf") {
            format!("{}.pdf", pdf_name)
        } else {
            pdf_name.to_string()
        };

        PdfResponse(
            Response::build()
                .header(ContentType::PDF)
                .header(Header::new(
                    "Content-Disposition",
                    format!("attachment; filename=\"{}\"", download_name)
                ))
                .sized_body(None, file.take_file())
                .finalize()
        )
    })
}

// ------------ Launch ------------

#[launch]
fn rocket() -> _ {
    env_logger::init();
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
        // mount our PDF-generation route at "/"
        .mount("/", routes![convert])
        // mount our public static files if we want
        .mount("/static", FileServer::from("static"))
        // mount our explicit download route for saved PDFs
        .mount("/download", routes![download_pdf])
}




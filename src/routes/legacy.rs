use crate::types::*;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::Either;
use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::Builder;

use crate::helpers::process_censor;

#[post("/", data = "<form>")]
pub async fn convert(
    form: Form<ConvertForm>,
) -> Result<Either<NamedFile, Json<ConvertResponse>>, ConvertError> {
    let form_data = form.into_inner();

    let processed_markdown = process_censor(&form_data.markdown);

    println!("Final processed markdown: {}", processed_markdown);

    // Start building pandoc command
    let mut pandoc_builder = Command::new("pandoc");
    pandoc_builder.arg("--from=markdown+raw_html");
    pandoc_builder.arg("--standalone");
    pandoc_builder.arg("--to=html5");

    pandoc_builder.arg("--variable=geometry:margin=1.5cm");
    pandoc_builder.arg("--variable=papersize=a4");

    // Create a temporary PDF
    let pdf_temp_path = Builder::new().suffix(".pdf").tempfile()?;
    let pdf_path = pdf_temp_path
        .path()
        .to_str()
        .expect("Non UTF-8 path not supported");
    pandoc_builder.arg(format!("--output={}", pdf_path));

    // Choose the PDF engine, default to Weasyprint
    let engine = form_data.engine.unwrap_or(PdfEngine::Weasyprint);
    pandoc_builder.arg(format!("--pdf-engine={}", engine));

    // Handle CSS
    let default_css_path = "templates/default.css";
    let default_css = fs::read_to_string(default_css_path)?;

    let css_content = if let Some(css) = form_data.css {
        format!("{}{}", default_css, css)
    } else {
        default_css
    };

    let mut css_file = Builder::new().suffix(".css").tempfile()?;
    css_file.write_all(css_content.as_bytes())?;
    let css_file_path = css_file.into_temp_path();
    let css_file_path_str = css_file_path.to_str().unwrap();
    pandoc_builder.arg(format!("--css={}", css_file_path_str));

    let mut temp_files = Vec::new();

    // Handle header template
    if let Some(header_template) = form_data.header_template {
        if !header_template.is_empty() {
            let current_dir = env::current_dir().unwrap();
            let header_path = current_dir.join("templates").join(&header_template);
            if header_path.exists() {
                let header_content = fs::read_to_string(&header_path)?;
                let mut header_file = Builder::new().suffix(".html").tempfile()?;
                header_file.write_all(header_content.as_bytes())?;
                let header_path = header_file.path().to_str().unwrap().to_string();
                pandoc_builder.arg(format!("--include-in-header={}", header_path));
                temp_files.push(header_file);
            } else {
                error!("Header template file not found at: {:?}", header_path);
                return Err(ConvertError::IO(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Header template file not found",
                )));
            }
        }
    }

    // Handle footer template
    if let Some(footer_template) = form_data.footer_template {
        if !footer_template.is_empty() {
            let current_dir = env::current_dir().unwrap();
            let footer_path = current_dir.join("templates").join(&footer_template);
            if footer_path.exists() {
                let footer_content = fs::read_to_string(&footer_path)?;
                let mut footer_file = Builder::new().suffix(".html").tempfile()?;
                footer_file.write_all(footer_content.as_bytes())?;
                let footer_path = footer_file.path().to_str().unwrap().to_string();
                pandoc_builder.arg(format!("--include-after-body={}", footer_path));
                temp_files.push(footer_file);
            } else {
                error!("Footer template file not found at: {:?}", footer_path);
                return Err(ConvertError::IO(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Footer template file not found",
                )));
            }
        }
    }

    pandoc_builder.stdin(Stdio::piped());
    pandoc_builder.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut pandoc_process = pandoc_builder.spawn()?;
    pandoc_process
        .stdin
        .as_mut()
        .unwrap()
        .write_all(processed_markdown.as_bytes())?;
    let output = pandoc_process.wait_with_output()?;

    // temp_files will be dropped here, after pandoc is done
    let _ = temp_files;

    if !output.status.success() {
        error!("Pandoc process failed with output: {:?}", output);
        return Err(ConvertError::Output(output));
    }

    if let (Some(client_id), Some(pdf_name)) = (form_data.client_id, form_data.pdf_name) {
        let client_dir = std::path::Path::new("public").join("pdf").join(&client_id);
        fs::create_dir_all(&client_dir)?;

        let final_pdf_name = if !pdf_name.ends_with(".pdf") {
            format!("{}.pdf", pdf_name)
        } else {
            pdf_name
        };

        let out_path = client_dir.join(&final_pdf_name);
        fs::copy(pdf_path, &out_path)?;

        let download_link = format!("/download/{}/{}", client_id, final_pdf_name);
        Ok(Either::Right(Json(ConvertResponse {
            download_url: download_link,
        })))
    } else {
        Ok(Either::Left(NamedFile::open(pdf_temp_path.path()).await?))
    }
}

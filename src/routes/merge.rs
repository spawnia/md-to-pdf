use crate::helpers;
use crate::types::*;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::Either;
use std::process::{Command, Stdio};
use tempfile::Builder;
use uuid::Uuid;

#[post("/merge", format = "json", data = "<req>")]
pub async fn merge(
    req: Json<MergeRequest>,
) -> Result<Either<NamedFile, Json<ConvertResponse>>, AppError> {
    let req = req.into_inner();

    if req.pdfs.len() < 2 {
        return Err(AppError::BadRequest(
            "At least 2 PDFs are required for merging".to_string(),
        ));
    }

    // Resolve all PDF paths
    let mut resolved_paths = Vec::new();
    for pdf_url in &req.pdfs {
        let path = helpers::resolve_pdf_path(pdf_url)?;
        resolved_paths.push(path);
    }

    // Use pdfunite to merge
    let output_temp = Builder::new().suffix(".pdf").tempfile()?;
    let output_path = output_temp
        .path()
        .to_str()
        .expect("Non UTF-8 path")
        .to_string();

    let mut cmd = Command::new("pdfunite");
    for path in &resolved_paths {
        cmd.arg(path.to_str().expect("Non UTF-8 path"));
    }
    cmd.arg(&output_path);

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(AppError::ProcessFailed {
            message: "PDF merge failed".to_string(),
            stderr,
        });
    }

    if let (Some(client_id), Some(pdf_name)) = (req.client_id, req.pdf_name) {
        let download_url = helpers::save_pdf(output_temp.path(), &client_id, &pdf_name)?;
        Ok(Either::Right(Json(ConvertResponse { download_url })))
    } else {
        // Generate a unique name for temp storage
        let client_id = "temp";
        let pdf_name = format!("merged-{}.pdf", Uuid::new_v4());
        let download_url = helpers::save_pdf(output_temp.path(), client_id, &pdf_name)?;
        Ok(Either::Right(Json(ConvertResponse { download_url })))
    }
}

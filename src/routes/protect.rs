use crate::helpers;
use crate::types::*;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::Either;
use std::process::{Command, Stdio};
use tempfile::Builder;

#[post("/protect", format = "json", data = "<req>")]
pub async fn protect(
    req: Json<ProtectRequest>,
) -> Result<Either<NamedFile, Json<ConvertResponse>>, AppError> {
    let req = req.into_inner();

    let source_path = helpers::resolve_pdf_path(&req.pdf)?;

    let output_temp = Builder::new().suffix(".pdf").tempfile()?;
    let output_path = output_temp
        .path()
        .to_str()
        .expect("Non UTF-8 path")
        .to_string();

    let output = Command::new("qpdf")
        .arg("--encrypt")
        .arg(&req.password)
        .arg(&req.password)
        .arg("256")
        .arg("--")
        .arg(source_path.to_str().expect("Non UTF-8 path"))
        .arg(&output_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(AppError::ProcessFailed {
            message: "PDF encryption failed".to_string(),
            stderr,
        });
    }

    if let (Some(client_id), Some(pdf_name)) = (req.client_id, req.pdf_name) {
        let download_url = helpers::save_pdf(output_temp.path(), &client_id, &pdf_name)?;
        Ok(Either::Right(Json(ConvertResponse { download_url })))
    } else {
        Ok(Either::Left(
            NamedFile::open(output_temp.path())
                .await
                .map_err(|e| AppError::Io(e))?,
        ))
    }
}

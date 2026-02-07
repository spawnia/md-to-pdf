use crate::helpers;
use crate::types::*;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::Either;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::Builder;

#[post("/watermark", format = "json", data = "<req>")]
pub async fn watermark(
    req: Json<WatermarkRequest>,
) -> Result<Either<NamedFile, Json<ConvertResponse>>, AppError> {
    let req = req.into_inner();

    let source_path = helpers::resolve_pdf_path(&req.pdf)?;
    let opacity = req.opacity.unwrap_or(0.06);
    let angle = req.angle.unwrap_or(-45.0);

    // Create a watermark overlay PDF using weasyprint
    let watermark_html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
<style>
@page {{ size: A4; margin: 0; }}
body {{
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  width: 100vw;
}}
.watermark {{
  font-size: 80pt;
  color: rgba(0, 0, 0, {opacity});
  transform: rotate({angle}deg);
  white-space: nowrap;
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%) rotate({angle}deg);
}}
</style>
</head>
<body>
<div class="watermark">{text}</div>
</body>
</html>"#,
        opacity = opacity,
        angle = angle,
        text = req.text
    );

    // Write watermark HTML to temp file
    let mut wm_html_file = Builder::new().suffix(".html").tempfile()?;
    wm_html_file.write_all(watermark_html.as_bytes())?;
    let wm_html_path = wm_html_file.into_temp_path();

    let wm_pdf_temp = Builder::new().suffix(".pdf").tempfile()?;
    let wm_pdf_path = wm_pdf_temp
        .path()
        .to_str()
        .expect("Non UTF-8 path")
        .to_string();

    // Generate watermark PDF with weasyprint
    let output = Command::new("weasyprint")
        .arg(wm_html_path.to_str().expect("Non UTF-8 path"))
        .arg(&wm_pdf_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(AppError::ProcessFailed {
            message: "Watermark PDF generation failed".to_string(),
            stderr,
        });
    }

    // Overlay watermark using qpdf
    let output_temp = Builder::new().suffix(".pdf").tempfile()?;
    let output_path = output_temp
        .path()
        .to_str()
        .expect("Non UTF-8 path")
        .to_string();

    let qpdf_output = Command::new("qpdf")
        .arg(source_path.to_str().expect("Non UTF-8 path"))
        .arg("--overlay")
        .arg(&wm_pdf_path)
        .arg("--")
        .arg(&output_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !qpdf_output.status.success() {
        let stderr = String::from_utf8_lossy(&qpdf_output.stderr).to_string();
        return Err(AppError::ProcessFailed {
            message: "Watermark overlay failed".to_string(),
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

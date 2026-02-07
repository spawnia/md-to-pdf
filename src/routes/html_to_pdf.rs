use crate::helpers;
use crate::types::*;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::Either;

#[post("/html-to-pdf", format = "json", data = "<req>")]
pub async fn html_to_pdf(
    req: Json<HtmlToPdfRequest>,
) -> Result<Either<NamedFile, Json<ConvertResponse>>, AppError> {
    let req = req.into_inner();

    let css_path = helpers::build_css(req.css.as_deref(), req.options.as_ref())?;
    let css_path_str = css_path.to_str().expect("Non UTF-8 path");

    let pdf_path = helpers::run_weasyprint(&req.html, css_path_str)?;

    if let (Some(client_id), Some(pdf_name)) = (req.client_id, req.pdf_name) {
        let download_url = helpers::save_pdf(pdf_path.as_ref(), &client_id, &pdf_name)?;
        Ok(Either::Right(Json(ConvertResponse { download_url })))
    } else {
        Ok(Either::Left(
            NamedFile::open(pdf_path.as_ref())
                .await
                .map_err(|e| AppError::Io(e))?,
        ))
    }
}

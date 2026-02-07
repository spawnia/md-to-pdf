use crate::helpers;
use crate::types::*;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::Either;
use tera::{Context, Tera};

#[post("/render", format = "json", data = "<req>")]
pub async fn render(
    req: Json<RenderRequest>,
) -> Result<Either<NamedFile, Json<ConvertResponse>>, AppError> {
    let req = req.into_inner();

    // Validate that data is a JSON object
    if !req.data.is_object() {
        return Err(AppError::BadRequest(
            "\"data\" must be a JSON object".to_string(),
        ));
    }

    // Render the Tera template with the provided data
    let context = Context::from_value(req.data)?;
    let rendered_html = Tera::one_off(&req.template, &context, true)?;

    let css_path = helpers::build_css(req.css.as_deref(), req.options.as_ref())?;
    let css_path_str = css_path.to_str().expect("Non UTF-8 path");

    // Use weasyprint directly (no pandoc needed for pre-rendered HTML)
    let pdf_path = helpers::run_weasyprint(&rendered_html, css_path_str)?;

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

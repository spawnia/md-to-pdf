use crate::helpers;
use crate::types::*;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::Either;

#[post("/convert", format = "json", data = "<req>")]
pub async fn convert(
    req: Json<ConvertRequest>,
) -> Result<Either<NamedFile, Json<ConvertResponse>>, AppError> {
    let req = req.into_inner();

    let processed_markdown = helpers::process_censor(&req.markdown);

    let css_path = helpers::build_css(req.css.as_deref(), req.options.as_ref())?;
    let css_path_str = css_path.to_str().expect("Non UTF-8 path");

    let engine = req.engine.unwrap_or_default();

    // Resolve header/footer (inline HTML takes priority over template file)
    let header_temp = helpers::resolve_header_footer(
        req.header_html.as_deref(),
        req.header_template.as_deref(),
    )?;
    let footer_temp = helpers::resolve_header_footer(
        req.footer_html.as_deref(),
        req.footer_template.as_deref(),
    )?;

    let pdf_path = helpers::run_pandoc(
        &processed_markdown,
        css_path_str,
        &engine,
        req.options.as_ref(),
        header_temp.as_ref().and_then(|p| p.to_str()),
        footer_temp.as_ref().and_then(|p| p.to_str()),
    )?;

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

use crate::helpers;
use crate::types::*;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use tera::{Context, Tera};

#[post("/preview", format = "json", data = "<req>")]
pub async fn preview(req: Json<PreviewRequest>) -> Result<(ContentType, Vec<u8>), AppError> {
    let req = req.into_inner();

    let css_path = helpers::build_css(req.css.as_deref(), req.options.as_ref())?;
    let css_path_str = css_path.to_str().expect("Non UTF-8 path");

    // Determine which input mode to use
    let pdf_path = if let Some(ref markdown) = req.markdown {
        // Markdown mode (like /api/convert)
        let processed = helpers::process_censor(markdown);
        let engine = req.engine.clone().unwrap_or_default();

        let header_temp = helpers::resolve_header_footer(
            req.header_html.as_deref(),
            req.header_template.as_deref(),
        )?;
        let footer_temp = helpers::resolve_header_footer(
            req.footer_html.as_deref(),
            req.footer_template.as_deref(),
        )?;

        helpers::run_pandoc(
            &processed,
            css_path_str,
            &engine,
            req.options.as_ref(),
            header_temp.as_ref().and_then(|p| p.to_str()),
            footer_temp.as_ref().and_then(|p| p.to_str()),
        )?
    } else if let (Some(ref template), Some(ref data)) = (&req.template, &req.data) {
        // Template mode (like /api/render)
        if !data.is_object() {
            return Err(AppError::BadRequest(
                "\"data\" must be a JSON object".to_string(),
            ));
        }
        let context = Context::from_value(data.clone())?;
        let rendered_html = Tera::one_off(template, &context, true)?;
        helpers::run_weasyprint(&rendered_html, css_path_str)?
    } else if let Some(ref html) = req.html {
        // HTML mode (like /api/html-to-pdf)
        helpers::run_weasyprint(html, css_path_str)?
    } else {
        return Err(AppError::BadRequest(
            "Provide one of: markdown, html, or template+data".to_string(),
        ));
    };

    let png_data = helpers::pdf_to_png(pdf_path.as_ref())?;
    Ok((ContentType::PNG, png_data))
}

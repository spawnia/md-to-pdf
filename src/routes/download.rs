use crate::types::PdfResponse;
use rocket::fs::NamedFile;
use rocket::http::{ContentType, Header};
use rocket::response::Response;
use std::path::Path;

#[get("/<client_id>/<pdf_name>")]
pub async fn download_pdf(client_id: &str, pdf_name: &str) -> Option<PdfResponse> {
    let path = Path::new("public")
        .join("pdf")
        .join(client_id)
        .join(pdf_name);

    NamedFile::open(path).await.ok().map(|file| {
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
                    format!("attachment; filename=\"{}\"", download_name),
                ))
                .sized_body(None, file.take_file())
                .finalize(),
        )
    })
}

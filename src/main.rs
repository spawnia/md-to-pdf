#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod helpers;
mod routes;
mod types;

use env_logger;
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

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
        // Legacy FormData endpoint (backward compatible)
        .mount("/", routes![routes::legacy::convert])
        // Static files
        .mount("/static", FileServer::from("static"))
        // Download saved PDFs
        .mount("/download", routes![routes::download::download_pdf])
        // New JSON API endpoints
        .mount(
            "/api",
            routes![
                routes::health::health,
                routes::convert::convert,
                routes::render::render,
                routes::html_to_pdf::html_to_pdf,
                routes::preview::preview,
                routes::merge::merge,
                routes::watermark::watermark,
                routes::protect::protect,
            ],
        )
}

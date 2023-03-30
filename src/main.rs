#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

mod db;
mod api;
mod auth_utils;
mod server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");

    // Start http server
    HttpServer::new(move || {
        let cors = Cors::permissive().allow_any_origin();
        let pool = db::util::establish_connection().expect("Couldn't create DB pool");

        App::new()
            .wrap(cors)
            .app_data(pool)
            .configure(server::init_api::init_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

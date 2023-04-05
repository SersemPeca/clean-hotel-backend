#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use std::{collections::HashMap, time::Duration};
use actix_rt::time;

mod db;
mod api;
mod auth_utils;
mod server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");

    actix_rt::spawn(async move {
        use crate::db::{
            models::room::NewRoom,
            crud::rooms::update_all_rooms_to_be_cleaned,
        };

        // 86400 seconds in a day
        let mut interval = time::interval(Duration::from_secs(86400));
        let new_pool = db::util::establish_connection().expect("Could not create DB pool");

        loop {
            interval.tick().await;
            update_all_rooms_to_be_cleaned(&new_pool);
        }
    });

    HttpServer::new(|| {
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

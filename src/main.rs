use std::env;
use actix_web::{App, HttpServer, Responder, get, web};
use serde::Serialize;
use dotenvy::dotenv;
use crate::{db::db::init_db, routes::task_routes::task_routes, routes::user_routes::user_routes};

mod utils;
mod routes;
mod models;
mod db;
mod controllers;
mod middlewares;

#[derive(Serialize)]
struct Response {
    message: String,
    status: u16,
}

#[get("/")]
async fn hello() -> impl Responder {
    let response = Response {
        message: "Hello world".to_string(),
        status: 201,
    };
    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let uri = env::var("MONGO_URI").expect("❌ failed to get url from dotenv");

    let port: u16 = env::var("PORT")
        .expect("❌ failed to get port from env")
        .parse()
        .expect("❌ PORT must be a number");

    let client = init_db(uri).await.expect("Failed to connect to MongoDB");

    let host = "127.0.0.1";

    println!("🚀 Server running at http://{}:{}", host, port);

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(user_routes)
            .configure(task_routes)
    })
    .bind((host, port))?
    .run()
    .await
}

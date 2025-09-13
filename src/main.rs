use std::sync::Mutex;

use crate::model::Item;
use actix_web::{App, HttpServer, web};
mod handlers;
mod model;

#[derive(Clone, Debug)]
pub struct AppState {
    pub items: Vec<Item>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Mutex::new(AppState { items: vec![] }));
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/api/items", web::post().to(handlers::create_item))
            .route("/api/items", web::get().to(handlers::get_items))
            .route("/api/items/{id}", web::get().to(handlers::get_item))
            .route("/api/items/{id}", web::put().to(handlers::update_item))
            .route(
                "/api/items/{id}",
                web::delete().to(handlers::delet_user_api),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

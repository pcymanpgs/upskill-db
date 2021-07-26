use std::collections::HashMap;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn hello() -> impl Responder {
    let mut resp = HashMap::new();
    resp.insert("hello", "db");
    HttpResponse::Ok().json(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(
            web::scope("/db")
                .route("", web::get().to(hello)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

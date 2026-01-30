use actix_web::{App, HttpResponse, HttpServer, web};
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn page() -> HttpResponse {
    HttpResponse::Ok().body("hello welcome to my page")
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/page", web::get().to(page))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

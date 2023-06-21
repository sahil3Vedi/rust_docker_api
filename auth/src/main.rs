use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn get_message() -> impl Responder {
    let message = serde_json::json!({
        "message": "let's get rusty!"
    });
    HttpResponse::Ok().json(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new().service(get_message))
        .bind("0.0.0.0:4000")?
        .run()
        .await
}
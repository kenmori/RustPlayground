use acitix_web::{web, App, HttpResponse, HttpServer, Responder}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world agein")
}


async fn main() -> std::io::result<()>{
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
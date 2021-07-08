fn main() {
    println!("Hello, world!");
}

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!";
}
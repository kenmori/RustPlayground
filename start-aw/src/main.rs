use actix_web::{server, App, HttpRequest, Responder};

fn hello(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("aWorld");
    format!("Hello{}!", to)
}
fn main() {
    server::new(|| {
        App::new().resource("/", |r| r.f(hello)).resource("/{name}", |r| r.f(hello))
    }).bind("localhost:3000").expect("Can not bind to port 30000").run();
}



extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |route| route.f(greet))
            .resource("/{name}", |route| route.f(greet))
    })
    .bind("127.0.0.1:5000")
    .expect("Can not bind to port 5000")
    .run();
}

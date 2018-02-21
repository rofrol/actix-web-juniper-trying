extern crate actix_web;
extern crate juniper;
use actix_web::*;
use actix_web::headers::ContentEncoding;

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn graphiql(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Identity) // Needed else actor panics at Option::unwrap
        .body(juniper::graphiql::graphiql_source("/test/graphql")).unwrap()
}

fn main() {
    HttpServer::new(
        || Application::new()
            .resource("/", |r| r.f(index))
            .resource("/test", |r| r.f(graphiql))
        )
        .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:8088")
        .run();
}
extern crate actix_web;
extern crate juniper;
use actix_web::*;
use actix_web::headers::ContentEncoding;
extern crate futures;
use futures::future::{Future, ok};

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn graphiql(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Identity) // Needed else actor panics at Option::unwrap
        .body(juniper::graphiql::graphiql_source("/test/graphql")).unwrap()
}

fn graphql_handle_post(request: HttpRequest) -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {
    println!("graphql_handle_post");
    Box::new(request.urlencoded().and_then(move |params| {
        ok(httpcodes::HTTPOk.with_body(format!("graphql_handle_post invoked with body {:?}", params)))
    }).map_err(actix_web::Error::from))
}

fn main() {
    HttpServer::new(
        || Application::new()
            .resource("/", |r| r.f(index))
            .resource("/test", |r| r.f(graphiql))
            .resource("/test/graphql", |r| r.method(Method::POST).a(graphql_handle_post))
        )
        .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:8088")
        .run();
}
extern crate lazy_static;

#[macro_use]
extern crate slog;
extern crate slog_term;

extern crate actix;
extern crate actix_web;
use actix_web::*;
use actix_web::headers::ContentEncoding;

extern crate futures;
use futures::future::{Future, ok};

extern crate serde_json;

extern crate juniper;

use slog::*;
use slog_term::*;

use std::io;

fn graphql_handle_post(request: HttpRequest) -> Box<Future<Item=HttpResponse, Error=actix_web::Error>> {

    let plain: PlainSyncDecorator<io::Stdout> = PlainSyncDecorator::new(io::stdout());
    let logger: Logger = Logger::root(
       FullFormat::new(plain)
           .build().fuse(), o!()
    );

    info!(logger, "graphql_handle_post");

    Box::new(request.urlencoded().and_then(move |params| {
        info!(logger.clone(), "graphql_handle_post invoked with body {:?}", params);
        ok(httpcodes::HTTPOk.with_body(format!("graphql_handle_post invoked with body {:?}", params)))
    }).map_err(actix_web::Error::from))
}

fn graphql_handle_get(
    request: HttpRequest,
) -> String {
    let query_string = request.query_string();

    let plain: PlainSyncDecorator<io::Stdout> = PlainSyncDecorator::new(io::stdout());
    let logger: Logger = Logger::root(
        FullFormat::new(plain)
            .build().fuse(), o!()
    );

    info!(logger, "request-parameter is {:?}", query_string);
    format!("request-parameter was {:?}", query_string).to_owned()
}

fn index(_: HttpRequest) -> &'static str {
    "Hello World!"
}

fn main() {
    let url = "localhost:8000";

    let plain: PlainSyncDecorator<io::Stdout> = PlainSyncDecorator::new(io::stdout());
    let logger: Logger = Logger::root(
        FullFormat::new(plain)
            .build().fuse(), o!()
    );

    let sys = actix::System::new("unnamed");

    HttpServer::new(
        || Application::new()
            .resource("/", |r| r.f(index))
            .resource("/test", |r| r.f(graphiql))
            .resource("/test/graphql?{request}", |r| r.method(Method::GET).f(graphql_handle_get))
            .resource("/test/graphql", |r| r.method(Method::POST).a(graphql_handle_post))
        )
        .bind(url).unwrap()
        .start();

    info!(logger, "Started http server at {}", url);

    sys.run();
}

fn graphiql(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Identity) // Needed else actor panics at Option::unwrap
        .body(juniper::graphiql::graphiql_source("/test/graphql")).unwrap()
}
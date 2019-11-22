// #[macro_use]
// extern crate juniper;

// use std::io;
use std::sync::Arc;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use listenfd::ListenFd;
use futures::future::Future;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;

mod models;
use crate::models::{create_schema, Schema};

fn main() { // -> io::Result<()>
  let schema = std::sync::Arc::new(create_schema());
  let mut listenfd = ListenFd::from_env();
  let mut server = HttpServer::new(move || {
    App::new()
      .data(schema.clone())
      .service(web::resource("/graphql").route(web::post().to_async(graphql)))
      .service(web::resource("/").route(web::get().to(playground)))
  });

  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l).unwrap()
  }
  else {
    server.bind("localhost:4000").unwrap()
  };

  server.run().unwrap();
}

fn graphql(
  st: web::Data<Arc<Schema>>,
  data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || {
    let res = data.execute(&st, &());
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .map_err(Error::from)
  .and_then(|user| {
    Ok(HttpResponse::Ok().content_type("application/json").body(user))
  })
}

fn playground() -> HttpResponse {
  let html = playground_source("http://localhost:4000/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}
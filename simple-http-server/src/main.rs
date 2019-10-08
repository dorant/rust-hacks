extern crate futures;
extern crate hyper;
extern crate hostname;

#[macro_use]
extern crate serde_json;

use hyper::service::service_fn_ok;
use hyper::{Body, Method, Response, Server};
use hostname::get_hostname;

// This is here because we use map_error.
use futures::Future;

use std::env;

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();

    let router = || {
        service_fn_ok(|req| match (req.method(), req.uri().path()) {
            (&Method::GET, "/healthz") => {
                Response::new(Body::from(json!({"status": "ok"}).to_string()))
            }
            (_, _) => {
                let msg = env::var("MESSAGE").unwrap_or_else(|_| "".to_string());
                Response::new(Body::from(
                    json!({"host": get_hostname().unwrap(),
                           "message": msg}).to_string()))

                // Specific status/response is needed, use:
                // let mut res = Response::new(Body::from("not found"));
                // *res.status_mut() = StatusCode::NOT_FOUND;
                // res
            }
        })
    };

    // Setup and run the server
    let server = Server::bind(&addr).serve(router);

    hyper::rt::run(server.map_err(|e| {
        eprintln!("server error: {}", e);
    }));
}

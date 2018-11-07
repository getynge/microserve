extern crate hyper;
extern crate futures;

use futures::{future, Future};
use std::alloc::System;
use std::fs::File;
use std::io::Read;
use hyper::{service::service_fn, Method, Body, Request, Response, Server, StatusCode};

#[global_allocator]
static GLOBAL: System = System;

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn get_file(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match req.method() {
        &Method::GET => {
            let uri = req.uri().path();
            let path;
            if uri.ends_with("/") {
                path = format!("./www/{}/index.html", uri)
            } else {
                path = format!("./www/{}", uri)
            }
            if let Ok(mut file) = File::open(path) {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => *response.body_mut() = Body::from(contents),
                    _ => *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR
                };
            } else {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
        _ => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        }
    }

    Box::new(future::ok(response))
}

fn main() {
    let addr = ([0, 0, 0, 0], 80).into();
    let server = Server::bind(&addr)
        .serve(|| service_fn(get_file))
        .map_err(|e| eprintln!("error: {}", e));

    println!("Serving on port 80");
    hyper::rt::run(server)
}
#[macro_use]
extern crate clap;
extern crate hyper;
extern crate futures;

use futures::{future, Future};
use std::alloc::System;
use std::fs::File;
use std::io::Read;
use hyper::{service::service_fn, Method, Body, Request, Response, Server, StatusCode};
use clap::App;

#[global_allocator]
static GLOBAL: System = System;

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn get_file(base_path: String, req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match req.method() {
        &Method::GET => {
            let uri = req.uri().path();
            let path;
            if uri.ends_with("/") {
                path = format!("{}/{}/index.html", base_path, uri)
            } else {
                path = format!("{}/{}", base_path, uri)
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
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let base_path = matches.value_of("base_path").unwrap_or("./www/").to_owned();
    let announced_path = base_path.clone();
    let port = matches.value_of("port").unwrap_or("80").parse::<u16>().unwrap_or(80);
    let addr = ([0, 0, 0, 0], port).into();

    let server = Server::bind(&addr)
        .serve(move || {
            let copy = base_path.clone();
            service_fn(move |r| get_file(copy.clone(), r))
        })
        .map_err(|e| eprintln!("error: {}", e));

    println!("Serving directory {} on port {}", announced_path, port);
    hyper::rt::run(server)
}
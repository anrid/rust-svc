#![deny(warnings)]

extern crate pretty_env_logger;

use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use rand::prelude::*;

async fn service(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut numbers = [0i32, 4];
    for x in &mut numbers {
        *x = rng.sample(distr);
    }

    Ok(Response::new(format!("Random numbers: {:?}", numbers).into()))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(service)) }
    });

    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{} ..", addr);
    server.await?;

    Ok(())
}
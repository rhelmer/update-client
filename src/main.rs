extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::str;

use futures::{Future, Stream};

use hyper::Client;
use hyper::{Method, Request};
use hyper::header::{ContentLength, ContentType};

use tokio_core::reactor::Core;

#[derive(Serialize)]
struct UpdateCheck {
    product: &'static str,
    version: i32,
    platform: &'static str,
    locale: &'static str,
}

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let check = UpdateCheck {
        product: "your_app",
        version: 666,
        platform: "macOS",
        locale: "en-US",
    };

    let json_post = serde_json::to_string(&check).unwrap();
    let uri = "http://localhost:8000/src/update.json".parse().unwrap();
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json_post.len() as u64));
    req.set_body(json_post);

    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());
        res.body().concat2()
    });

    let posted = core.run(post).unwrap();

    println!("POST: {}", str::from_utf8(&posted).unwrap());
}

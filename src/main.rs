extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::str;
use std::fs::File;
use std::io::prelude::*;

use futures::{Future, Stream};

use hyper::Client;
use hyper::{Method, Request};
use hyper::header::{ContentLength, ContentType};

use tokio_core::reactor::Core;

#[derive(Serialize)]
struct AppConfig {
    product: &'static str,
    version: i32,
    platform: &'static str,
    locale: &'static str,
}

// FIXME this should be a vec of updates.
#[derive(Serialize, Deserialize, Debug)]
struct Update {
    update_type: &'static str,
    url: &'static str,
    hash_function: &'static str,
    hash_value: &'static str,
    size: i32,
    version: i32,
}

fn update_check(app_config: AppConfig, uri: hyper::Uri) -> Update {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let json_post = serde_json::to_string(&app_config).unwrap();
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json_post.len() as u64));
    req.set_body(json_post);

    let post = client.request(req).and_then(|res| {
        res.body().concat2()
    });

    let posted = core.run(post).unwrap();

    // FIXME mock update response for now.
    let update = Update {
        update_type: "blocklist",
        url: "http://localhost:8080/src/blah.zip",
        hash_function: "sha512",
        hash_value: "abc123",
        size: 1024,
        version: 1000,
    };

    update
}

/// Download assets from an update and verify that metdata
/// matches.
fn download_update(update: Update) -> String {
    let uri = update.url.parse().unwrap();
    let update_type = update.update_type;
    let version = update.version;

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let req = Request::new(Method::Post, uri);
    let asset = client.request(req).and_then(|res| {
        res.body().concat2()
    });

    let data = core.run(asset).unwrap();

    let download_filename = format!("/tmp/{}_{}.zip", update_type, version);
    let mut download_file = File::create(&download_filename).unwrap();

    // FIXME fake data for now.
    download_file.write_all(b"fake data").unwrap();

    // FIXME verify asset.

    download_filename
}



fn main() {
    let uri = "http://localhost:8000/src/update.json".parse().unwrap();

    // FIXME hardcode update check values for now.
    let app_config = AppConfig {
        product: "your_app",
        version: 666,
        platform: "macOS",
        locale: "en-US",
    };

    let available_update = update_check(app_config, uri);
    let download_path = download_update(available_update);
    println!("{}", download_path);
}

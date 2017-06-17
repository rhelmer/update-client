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

// FIXME share all these structs w/ server.
#[derive(Serialize)]
struct AppConfig {
    product: String,
    version: i32,
    platform: String,
    locale: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Update {
    update_type: String,
    url: String,
    hash_function: String,
    hash_value: String,
    size: i32,
    version: i32,
}

#[derive(Serialize)]
struct ResultMessage {
    update_type: String,
    download_path: String,
}

#[derive(Serialize)]
struct SuccessUpdateStatus {
    update_type: String,
    version: i32,
}

#[derive(Serialize)]
struct FailedUpdateStatus {
    update_type: String,
    url: String,
    hash_function: String,
    hash_value: String,
    size: i32,
    version: i32,
    download_path: String,
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

    let post_response = core.run(post).unwrap();

    let json_str = str::from_utf8(&post_response).unwrap();
    let update: Update = serde_json::from_str(&json_str).unwrap();

    // FIXME should return a vec of updates.
    update
}

/// Download assets from an update and verify that metdata
/// matches.
fn download_update(update: &Update) -> String {
    let uri = update.url.parse().unwrap();
    let update_type = &update.update_type;
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

    // FIXME verify asset using hash and size from metadata.
    download_filename
}



fn main() {
    let uri = "http://localhost:8000/src/update.json".parse().unwrap();

    // FIXME hardcode update check values for now.
    let app_config = AppConfig {
        product: "your_app".to_string(),
        version: 666,
        platform: "macOS".to_string(),
        locale: "en-US".to_string(),
    };

    let available_update = update_check(app_config, uri);
    let download_path = download_update(&available_update);

    let result_message = ResultMessage {
        update_type: available_update.update_type,
        download_path: download_path,
    };

    // FIXME should return a vec of ResultMessages.
    let output = serde_json::to_string(&result_message).unwrap();
    println!("{}", output);
}

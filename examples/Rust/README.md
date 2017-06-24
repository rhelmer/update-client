# Example app for Update client

This is a simple Rust app to exercise the [Update client](https://github.com/rhelmer/update-client).

## Getting started

### Dependencies

You need to have an update server running. The default is the [Update server](https://github.com/rhelmer/update-server).

A simple `cargo run` should be enough to get the server running on `localhost:8000`.

Next, build the [Update client](https://github.com/rhelmer/update-client) by running `cargo build --release`.

### Build and run.

`cargo run`

The example app will attempt to spawn the update client as a child process.

This app will look in `../../target/release/update_client` for the update client binary, so make sure it was built with `--release` in the "Dependencies" section above!

If updates are found, the update client will download them and notify the
example app, which will "install" the update by copying it to the `./updates`
directory.

# Update client

A standalone client for any type of update (data, binary, application, ...)

This client is intended to either run as a Windows service,
or be started by the application. In either case, this client will
establish an inter-process communication (IPC) channel to notify
the application of available updates.

For the moment, the client simply uses stdin/stdout as the communications
channel.

## Getting started

Run an application server. The supported [Update server](https://github.com/rhelmer/update-server#readme) is recommended, but any server that implements the [update protocol](https://github.com/rhelmer/update-server#protocol) will work.

### Build and run.

`cargo run`

The client will attempt to connect to port 8000 on localhost,
and will send an update request.

If updates are available, the client will attempt to download
and apply them on the local system and send a "completion"
document to the server.

In the future, the client will send an IPC message to the application
indicating which update(s) have been applied and are ready to use.

For the moment, the message is returned on stdout.

If no updates are available, the client will exit if running in
standalone mode. When running as an OS service is supported, the request
will be re-tried at the configured interval.

### Using the update client from an application.

The `./examples` directory contains simple applications written in
different programming languages that show how to use the Application Update
client.

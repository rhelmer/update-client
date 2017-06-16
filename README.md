Firefox Update Client
=====================

A standalone client for Firefox updates.

This client is intended to either run as a Windows service,
or be started by Firefox. In either case, this client will
establish an inter-process communication (IPC) channel to notify
Firefox of available updates.

The protocol is inspired by Google's Omaha client+server,
but is modernized and simplified.

Getting started
===============

# Build and run.
`cargo run`

The client will attempt to connect to port 9999 on localhost,
and will send an update request.

See the server documentation for a description of the protocol.

If updates are available, the client will attempt to download
and apply them on the local system and send a "completion"
document to the server.  The client will then send an IPC
message to Firefox indicating which update(s) have been applied
and are ready to use.

If not updates are available, the client will exit if running in
standalone mode. If running as a Windows service then the request
will be re-tried at the configured interval.

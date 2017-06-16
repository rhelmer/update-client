Firefox Update Client
=====================

A standalone client for Firefox updates.

This client is intended to either run as a Windows service,
or be started by Firefox. In either case, this client will
establish an interprocess communication channel to notify
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

If updates are available, the client will attempt to apply them
on the local system and send a "completion" document to the server.

If not, the client will do nothing and terminate.

# A Vert.x Event-bus library for Rust built with Tokio

[![Build Status](https://travis-ci.org/PierreZ/vertx-rs.svg?branch=master)](https://travis-ci.org/PierreZ/vertx-rs)

## What is Vert.x?

[Vert.x](http://vertx.io/) is a tool-kit for building reactive applications on the JVM.

## What is the event-bus-bridge thing?

Taken from the official [documentation](http://vertx.io/docs/vertx-tcp-eventbus-bridge/java/):
> The TCP EventBus bridge as its name states is a bridge built on top of TCP, meaning that any application able to create sockets can use the EventBus from a remote Vert.x instance.

The idea of the project is to provide a set of libraries to ease the integration of Vert.x applications with applications developed in Rust. The idea was taken from [Google Summer of Code Ideas](http://vertx.io/gsoc/).

## What is Tokio?

[Tokio is a platform for writing fast networking code in Rust.](https://tokio.rs)

### The protocol

The protocol is quite simple:

* 4bytes int32 message length (big endian encoding)
* json string (encoded in UTF-8)

#### Messages from the bridge -> client

Every JSON message will include a `type` key, with a string
value. Each type is shown below, along with the companion keys for
that type:

#####  `type: "err"`

* `message`: (string, required) The type of error, one of:
  `"access_denied"`, `"address_required"`, `"unknown_address"`,
  `"unknown_type"`

##### `type: "message"`

For a regular message, the object will also contain:

* `address`: (string, required) Destination address.
* `body`: (object, required) Message content as a JSON object.
* `headers`: (object, optional) Headers as a JSON object with String values.
* `replyAddress`: (string, optional) Address for replying to.

When a message from the client requests a reply, and that reply fails,
the object will instead contain:

* `failureCode`: (number, required) The failure code
* `failureType`: (string, required) The failure name
* `message`: (string, required) The message from the exception that signaled the failure

#### Messages from the client -> bridge

The JSON object must contain a `type` key with a string value.  Each
type is shown below, along with the companion keys for that type:

##### `type: "send"`, `type: "publish"`

* `address`: (string, required) Destination address
* `body`: (object, required) Message content as a JSON object.
* `headers`: (object, optional) Headers as a JSON object with String values.
* `replyAddress`: (string, optional) Address for replying to.

##### `type: "register"`, `type: "unregister"`

* `address`: (string, required) Destination address
* `headers`: (object, optional) Headers as a JSON object with String values.

##### `type: "ping"`

`ping` requires no additional keys.

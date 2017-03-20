//! Vertx-rs, A Vert.x Event-bus library for Rust built with Tokio

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate futures;

extern crate rustc_serialize;

mod client;

#[test]
fn it_works() {}

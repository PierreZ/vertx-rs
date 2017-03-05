//! Vertx-rs, A Vert.x Event-bus library for Rust built with Tokio

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

pub mod codec;

extern crate rustc_serialize;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

#[test]
fn it_works() {}

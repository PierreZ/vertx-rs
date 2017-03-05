# A Vert.x Event-bus library for Rust

[![Build Status](https://travis-ci.org/PierreZ/vertx-rs.svg?branch=master)](https://travis-ci.org/PierreZ/vertx-rs)

## What is Vert.x?

[Vert.x](http://vertx.io/) is a tool-kit for building reactive applications on the JVM.

## What is the event-bus-bridge thing?

Taken from the official [documentation](http://vertx.io/docs/vertx-tcp-eventbus-bridge/java/):
> The TCP EventBus bridge as its name states is a bridge built on top of TCP, meaning that any application able to create sockets can use the EventBus from a remote Vert.x instance.

The idea of the project is to provide a set of libraries to ease the integration of Vert.x applications with applications developed in Rust. The idea was taken from [Google Summer of Code Ideas](http://vertx.io/gsoc/).

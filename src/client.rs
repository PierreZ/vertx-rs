//! client is the actual API for this library

use tokio_io::{io, AsyncRead, AsyncWrite};
use tokio_io::codec::length_delimited;
use tokio_proto::pipeline::ServerProto;


// Transport is handling the delimited Header in every Vert.x messages
struct Transport {}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Transport {
    // Implementing length_delimited allows us to not care about length delimited framing
    // https://docs.rs/tokio-io/0.1.0/tokio_io/codec/length_delimited/index.html
    fn bind_transport<U: AsyncRead + AsyncWrite>(io: U) -> length_delimited::Framed<T> {

        // Using this options allows us to decode as such:
        //         INPUT                        DECODED
        //+-- len ---+--- Payload ---+     +--- Payload ---+
        //| \x00\x0B |  Hello world  | --> |  Hello world  |
        //+----------+---------------+     +---------------+
        length_delimited::Builder::new()
            .length_field_offset(0) // default value
            .length_field_length(2)
            .length_adjustment(0)   // default value
            .new_read(io);
    }
}

use futures::{future, Future, BoxFuture};
use std::{io, str};
use tokio_service::Service;

pub struct TCPBridge;

impl Service for TCPBridge {
    type Request = VertxMessage;
    type Response = VertxMessage;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: String) -> Self::Future {
        self.inner.call(req)
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
/// Represents the actual Vertx message
/// TODO: missing headers and body as `JSONObject`
pub struct VertxMessage {
    type_msg: String,
    address: String,
    reply_address: String,
}
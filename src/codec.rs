use std::io;
use tokio_core::io::{Codec, EasyBuf};
use rustc_serialize::json;

pub struct LineCodec;

impl Codec for LineCodec {
    type In = VertxMessage;
    type Out = VertxMessage;

    #[allow(unused_variables)]
    // TODO
    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        Ok(None)
    }

    // Encode is transforming a VertxMessage into the right protocol:
    // * 4bytes int32 message length (big endian encoding)
    //* json string (encoded in UTF-8)
    fn encode(&mut self, msg: VertxMessage, buf: &mut Vec<u8>) -> io::Result<()> {
        let serialized = json::encode(&msg).unwrap();
        buf.push(serialized.len() as u8);
        buf.extend(serialized.as_bytes());
        Ok(())
    }
}

#[derive(RustcDecodable, RustcEncodable)]
// TODO: missing headers and body as JSONObject
pub struct VertxMessage {
    type_msg: String,
    address: String,
    reply_address: String,
}

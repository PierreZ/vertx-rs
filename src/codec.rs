//! codec module is implementing the codec part of Tokio

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

use byteorder::{BigEndian, ReadBytesExt};
use rustc_serialize::json;
use std::io;
use std::io::Cursor;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

#[derive(Clone, Copy, Debug)]
/// `LineCodec` is the struct implementing the Codec for Tokio
pub struct LineCodec;

impl Codec for LineCodec {
    type In = VertxMessage;
    type Out = VertxMessage;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        let mut copy = buf.clone();
        copy.split_off(4);
        // Copy now contains the 4 bytes with the size of the JSON. We don't need the head

        let mut cursor = Cursor::new(copy);
        let num = cursor.read_u32::<BigEndian>().unwrap();
        if buf.len() >= num as usize {
            // Buffer is holding everything we need, flushing it
            buf.drain_to(4); // Removing length
            let json = buf.drain_to(num as usize); // removing json
            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(json.as_slice()) {
                Ok(s) => {
                    let decoded: VertxMessage = json::decode(s).unwrap();
                    Ok(Some(decoded))
                }
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }

    /// Encode is transforming a VertxMessage into the right protocol:
    /// * 4bytes int32 message length (big endian encoding)
    /// * json string (encoded in UTF-8)
    fn encode(&mut self, msg: VertxMessage, buf: &mut Vec<u8>) -> io::Result<()> {
        let serialized = json::encode(&msg).unwrap();
        buf.push(serialized.len() as u8);
        buf.extend(serialized.as_bytes());
        Ok(())
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

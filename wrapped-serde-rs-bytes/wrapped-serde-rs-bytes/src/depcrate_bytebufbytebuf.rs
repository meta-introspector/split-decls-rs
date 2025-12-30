// Generated macro for ByteBuf (struct)
macro_rules! Depcrate_bytebufByteBuf {
() => {
// Module: crate::bytebuf
// Provides: {"ByteBuf"}
// Dependencies: {}
# [doc = " Wrapper around `Vec<u8>` to serialize and deserialize efficiently."] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = " use std::io;"] # [doc = ""] # [doc = " use serde_bytes::ByteBuf;"] # [doc = ""] # [doc = " fn deserialize_bytebufs() -> Result<(), bincode::error::DecodeError> {"] # [doc = "     let example_data = [2, 2, 3, 116, 119, 111, 1, 3, 111, 110, 101];"] # [doc = ""] # [doc = "     let map: HashMap<u32, ByteBuf>;"] # [doc = "     (map, _) = bincode::serde::decode_from_slice("] # [doc = "         &example_data,"] # [doc = "         bincode::config::standard(),"] # [doc = "     )?;"] # [doc = ""] # [doc = "     println!(\"{:?}\", map);"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     deserialize_bytebufs().unwrap();"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Default , Eq , Ord)] pub struct ByteBuf { bytes : Vec < u8 > , }
};
}

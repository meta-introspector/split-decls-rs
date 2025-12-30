// Generated macro for ByteArray (struct)
macro_rules! Depcrate_bytearrayByteArray {
() => {
// Module: crate::bytearray
// Provides: {"ByteArray"}
// Dependencies: {}
# [doc = " Wrapper around `[u8; N]` to serialize and deserialize efficiently."] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = " use std::io;"] # [doc = ""] # [doc = " use serde_bytes::ByteArray;"] # [doc = ""] # [doc = " fn deserialize_bytearrays() -> Result<(), bincode::error::DecodeError> {"] # [doc = "     let example_data = [2, 2, 3, 116, 119, 111, 1, 3, 111, 110, 101];"] # [doc = ""] # [doc = "     let map: HashMap<u32, ByteArray<3>>;"] # [doc = "     (map, _) = bincode::serde::decode_from_slice("] # [doc = "         &example_data,"] # [doc = "         bincode::config::standard(),"] # [doc = "     )?;"] # [doc = ""] # [doc = "     println!(\"{:?}\", map);"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     deserialize_bytearrays().unwrap();"] # [doc = " # }"] # [doc = " ```"] # [derive (Copy , Clone , Eq , Ord)] # [repr (transparent)] pub struct ByteArray < const N : usize > { bytes : [u8 ; N] , }
};
}

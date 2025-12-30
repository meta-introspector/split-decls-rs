// Generated macro for Bytes (struct)
macro_rules! Depcrate_bytesBytes {
() => {
// Module: crate::bytes
// Provides: {"Bytes"}
// Dependencies: {}
# [doc = " Wrapper around `[u8]` to serialize and deserialize efficiently."] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashMap;"] # [doc = " use std::io;"] # [doc = ""] # [doc = " use serde_bytes::Bytes;"] # [doc = ""] # [doc = " fn print_encoded_cache() -> Result<(), bincode::error::EncodeError> {"] # [doc = "     let mut cache = HashMap::new();"] # [doc = "     cache.insert(3, Bytes::new(b\"three\"));"] # [doc = "     cache.insert(2, Bytes::new(b\"two\"));"] # [doc = "     cache.insert(1, Bytes::new(b\"one\"));"] # [doc = ""] # [doc = "     bincode::serde::encode_into_std_write("] # [doc = "         &cache,"] # [doc = "         &mut io::stdout(),"] # [doc = "         bincode::config::standard(),"] # [doc = "     )?;"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     print_encoded_cache().unwrap();"] # [doc = " # }"] # [doc = " ```"] # [derive (Eq , Ord)] # [repr (transparent)] pub struct Bytes { bytes : [u8] , }
};
}

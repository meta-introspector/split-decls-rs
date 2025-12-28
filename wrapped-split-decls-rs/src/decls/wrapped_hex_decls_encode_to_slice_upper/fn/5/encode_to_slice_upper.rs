use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Encodes some bytes into a mutable slice of bytes using uppercase characters."] # [doc = ""] # [doc = " The output buffer, has to be able to hold exactly `input.len() * 2` bytes,"] # [doc = " otherwise this function will return an error."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use hex::FromHexError;"] # [doc = " # fn main() -> Result<(), FromHexError> {"] # [doc = " let mut bytes = [0u8; 4 * 2];"] # [doc = ""] # [doc = " hex::encode_to_slice_upper(b\"kiwi\", &mut bytes)?;"] # [doc = " assert_eq!(&bytes, b\"6B697769\");"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] pub fn encode_to_slice_upper < T : AsRef < [u8] > > (input : T , output : & mut [u8] ,) -> Result < () , FromHexError > { encode_to_slice_inner (input . as_ref () , output , HEX_CHARS_UPPER) }
}
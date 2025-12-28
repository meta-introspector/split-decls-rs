use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < const N : usize > FromHex for [u8 ; N] { type Error = FromHexError ; fn from_hex < T : AsRef < [u8] > > (hex : T) -> Result < Self , Self :: Error > { let mut out = [0_u8 ; N] ; decode_to_slice (hex , & mut out as & mut [u8]) ? ; Ok (out) } }
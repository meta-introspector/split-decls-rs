use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: encode_to_slice_inner");
fn encode_to_slice_inner (input : & [u8] , output : & mut [u8] , table : & [u8 ; 16] ,) -> Result < () , FromHexError > { if input . len () * 2 != output . len () { return Err (FromHexError :: InvalidStringLength) ; } for (byte , output) in input . iter () . zip (output . chunks_exact_mut (2)) { let (high , low) = byte2hex (* byte , table) ; output [0] = high ; output [1] = low ; } Ok (()) }
}
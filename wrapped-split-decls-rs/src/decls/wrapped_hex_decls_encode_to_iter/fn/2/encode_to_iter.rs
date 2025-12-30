use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: encode_to_iter");
fn encode_to_iter < T : iter :: FromIterator < char > > (table : & 'static [u8 ; 16] , source : & [u8]) -> T { BytesToHexChars :: new (source , table) . collect () }
}
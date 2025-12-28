use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn encode_to_iter < T : iter :: FromIterator < char > > (table : & 'static [u8 ; 16] , source : & [u8]) -> T { BytesToHexChars :: new (source , table) . collect () }
}
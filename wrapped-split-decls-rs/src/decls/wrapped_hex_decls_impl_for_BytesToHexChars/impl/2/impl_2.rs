use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > BytesToHexChars < 'a > { fn new (inner : & 'a [u8] , table : & 'static [u8 ; 16]) -> BytesToHexChars < 'a > { BytesToHexChars { inner : inner . iter () , table , next : None , } } }
}
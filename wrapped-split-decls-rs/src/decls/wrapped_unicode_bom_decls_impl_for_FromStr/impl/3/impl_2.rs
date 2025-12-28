use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FromStr for Bom { # [doc = " A `std::io::Error` instance returned by `std::fs::File::open`."] type Err = Error ; # [doc = " Parse the BOM type from the file located at `path`."] fn from_str (path : & str) -> Result < Self , Self :: Err > { let mut file = File :: open (path) ? ; Ok (Bom :: from (& mut file)) } }
}
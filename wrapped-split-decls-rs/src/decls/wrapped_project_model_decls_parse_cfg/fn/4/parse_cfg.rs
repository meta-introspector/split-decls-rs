use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: parse_cfg");
fn parse_cfg (s : & str) -> Result < cfg :: CfgAtom , String > { let res = match s . split_once ('=') { Some ((key , value)) => { if ! (value . starts_with ('"') && value . ends_with ('"')) { return Err (format ! ("Invalid cfg ({s:?}), value should be in quotes")) ; } let key = intern :: Symbol :: intern (key) ; let value = intern :: Symbol :: intern (& value [1 .. value . len () - 1]) ; cfg :: CfgAtom :: KeyValue { key , value } } None => cfg :: CfgAtom :: Flag (intern :: Symbol :: intern (s)) , } ; Ok (res) }
}
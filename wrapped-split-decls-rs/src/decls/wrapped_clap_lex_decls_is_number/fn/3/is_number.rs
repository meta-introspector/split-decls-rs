use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: is_number");
fn is_number (arg : & str) -> bool { let mut seen_dot = false ; let mut position_of_e = None ; for (i , c) in arg . as_bytes () . iter () . enumerate () { match c { b'0' ..= b'9' => { } b'.' if ! seen_dot && position_of_e . is_none () && i > 0 => seen_dot = true , b'e' | b'E' if position_of_e . is_none () && i > 0 => position_of_e = Some (i) , _ => return false , } } match position_of_e { Some (i) => i != arg . len () - 1 , None => true , } }
}
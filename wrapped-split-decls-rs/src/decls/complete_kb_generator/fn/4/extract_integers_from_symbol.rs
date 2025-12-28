use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: extract_integers_from_symbol");
fn extract_integers_from_symbol (symbol : & str) -> Vec < i32 > { let mut integers = Vec :: new () ; let mut current_num = String :: new () ; for ch in symbol . chars () { if ch . is_ascii_digit () { current_num . push (ch) ; } else { if ! current_num . is_empty () { if let Ok (num) = current_num . parse :: < i32 > () { integers . push (num) ; } current_num . clear () ; } } } if ! current_num . is_empty () { if let Ok (num) = current_num . parse :: < i32 > () { integers . push (num) ; } } integers }
}
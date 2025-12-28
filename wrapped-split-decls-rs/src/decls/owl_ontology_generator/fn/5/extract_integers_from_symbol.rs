use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn extract_integers_from_symbol (symbol : & str) -> Vec < i32 > { let mut integers = Vec :: new () ; let mut current_num = String :: new () ; let mut is_negative = false ; for (i , ch) in symbol . chars () . enumerate () { if ch == '-' && (i == 0 || ! symbol . chars () . nth (i - 1) . unwrap_or (' ') . is_ascii_digit ()) { is_negative = true ; } else if ch . is_ascii_digit () { current_num . push (ch) ; } else { if ! current_num . is_empty () { if let Ok (num) = current_num . parse :: < i32 > () { let final_num = if is_negative { - num } else { num } ; integers . push (final_num) ; } current_num . clear () ; is_negative = false ; } } } if ! current_num . is_empty () { if let Ok (num) = current_num . parse :: < i32 > () { let final_num = if is_negative { - num } else { num } ; integers . push (final_num) ; } } integers }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Split camelCase into separate terms"] fn split_camel_case (s : & str) -> Vec < String > { let mut terms = Vec :: new () ; let mut current = String :: new () ; for c in s . chars () { if c . is_uppercase () && ! current . is_empty () { terms . push (current . clone ()) ; current . clear () ; } current . push (c) ; } if ! current . is_empty () { terms . push (current) ; } terms }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Extract terms from a declaration name"] fn extract_terms (name : & str) -> Vec < String > { let mut terms = Vec :: new () ; for part in name . split ('_') { if ! part . is_empty () { terms . push (part . to_lowercase ()) ; let camel_terms = split_camel_case (part) ; for term in camel_terms { if term . len () >= 2 { terms . push (term . to_lowercase ()) ; } } } } terms . sort () ; terms . dedup () ; terms }
}
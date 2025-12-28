use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn get_language_from_extension (extension : & str) -> Option < & 'static str > { let mapping = get_language_mapping () ; for (language , extensions) in mapping . iter () { if extensions . contains (& extension) { return Some (language) ; } } None }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl iter::FromIterator<String> for SmolStr {
    fn from_iter<I: iter::IntoIterator<Item = String>>(iter: I) -> SmolStr {
        build_from_str_iter(iter.into_iter())
    }
}

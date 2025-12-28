use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for TimesRange { fn default () -> TimesRange { TimesRange (0 .. usize :: MAX) } }
}
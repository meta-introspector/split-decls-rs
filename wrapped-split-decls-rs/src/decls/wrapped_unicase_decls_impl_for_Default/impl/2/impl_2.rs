use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S : AsRef < str > + Default > Default for UniCase < S > { fn default () -> Self { Self :: new (Default :: default ()) } }
}
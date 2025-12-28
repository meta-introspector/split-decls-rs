use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , T : ? Sized > Captures < 'a > for T { }
}
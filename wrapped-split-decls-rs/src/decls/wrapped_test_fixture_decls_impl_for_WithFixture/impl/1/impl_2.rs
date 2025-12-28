use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < DB : ExpandDatabase + SourceDatabase + Default + 'static > WithFixture for DB { }
}
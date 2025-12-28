use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , I > ! IntoIterator for UnordItems < T , I > { }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl BuildHasher for FxBuildHasher { type Hasher = FxHasher ; fn build_hasher (& self) -> FxHasher { FxHasher :: default () } }
}
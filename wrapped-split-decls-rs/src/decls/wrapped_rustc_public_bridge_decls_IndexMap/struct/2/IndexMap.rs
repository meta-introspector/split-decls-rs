use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Similar to rustc's `FxIndexMap`, `IndexMap` with extra"] # [doc = " safety features added."] pub struct IndexMap < K , V > { index_map : fx :: FxIndexMap < K , V > , }
}
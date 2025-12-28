use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct BuiltLint { lint : & 'static Lint , groups : Vec < & 'static str > , }
}
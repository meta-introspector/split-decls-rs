use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ArgsWrapper { args : Arc < Object > , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Debug)] pub struct Dominators < Node : Idx > { kind : Kind < Node > , }
}
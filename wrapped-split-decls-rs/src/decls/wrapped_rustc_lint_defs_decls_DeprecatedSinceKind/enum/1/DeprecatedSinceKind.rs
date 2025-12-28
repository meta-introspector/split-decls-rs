use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone)] pub enum DeprecatedSinceKind { InEffect , InFuture , InVersion (String) , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default , Serialize , Deserialize)] struct WrappingConfig { # [serde (default)] crates : Vec < String > , }
}
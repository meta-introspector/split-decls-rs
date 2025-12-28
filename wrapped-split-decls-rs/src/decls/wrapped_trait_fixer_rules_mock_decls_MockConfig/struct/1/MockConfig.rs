use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize)] pub struct MockConfig { pub rule : Vec < Rule > , }
}
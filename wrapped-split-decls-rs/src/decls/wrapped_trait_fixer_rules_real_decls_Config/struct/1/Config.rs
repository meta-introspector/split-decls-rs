use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize)] pub struct Config { pub rule : Vec < Rule > , }
}
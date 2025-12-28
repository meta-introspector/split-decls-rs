use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default)] pub struct TermCollector { pub terms : Vec < Term > , }
}
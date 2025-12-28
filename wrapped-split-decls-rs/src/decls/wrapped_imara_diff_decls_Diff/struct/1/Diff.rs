use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] pub struct Diff { removed : Vec < bool > , added : Vec < bool > , }
}
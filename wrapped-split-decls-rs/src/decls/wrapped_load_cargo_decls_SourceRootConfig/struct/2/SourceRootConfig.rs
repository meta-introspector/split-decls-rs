use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default , Debug)] pub struct SourceRootConfig { pub fsc : FileSetConfig , pub local_filesets : Vec < u64 > , }
}
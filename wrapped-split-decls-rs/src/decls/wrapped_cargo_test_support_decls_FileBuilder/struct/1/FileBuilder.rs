use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (PartialEq , Clone)] struct FileBuilder { path : PathBuf , body : String , executable : bool , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FileRange = FileRangeWrapper < FileId > ;
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FileEncodeResult = Result < usize , (PathBuf , io :: Error) > ;
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type FileEncodeResult = Result < usize , (PathBuf , io :: Error) > ;
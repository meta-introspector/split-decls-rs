use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < PathBuf > for FileName { fn from (p : PathBuf) -> Self { FileName :: Real (RealFileName :: LocalPath (p)) } }
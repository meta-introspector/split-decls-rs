use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn path (p : & str) -> PathBuf { path_str (p) . into () }
}
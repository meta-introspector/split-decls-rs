use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn path (p : & str) -> PathBuf { path_str (p) . into () }
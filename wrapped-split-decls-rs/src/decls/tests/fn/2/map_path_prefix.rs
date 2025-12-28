use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn map_path_prefix (mapping : & FilePathMapping , p : & str) -> String { mapping . map_prefix (path (p)) . 0 . to_string_lossy () . to_string () }
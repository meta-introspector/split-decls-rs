use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn target_dir () -> Utf8PathBuf { match std :: env :: var ("CARGO_TARGET_DIR") { Ok (target) => Utf8PathBuf :: from (target) , Err (_) => project_root () . join ("target") , } }
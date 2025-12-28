use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Returns `true` if the local filesystem has low-resolution mtimes."] pub fn is_coarse_mtime () -> bool { cfg ! (emulate_second_only_system) || cfg ! (target_os = "macos") && is_ci () }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (unix)] fn home_dir_inner () -> Option < PathBuf > { # [allow (deprecated)] std :: env :: home_dir () }
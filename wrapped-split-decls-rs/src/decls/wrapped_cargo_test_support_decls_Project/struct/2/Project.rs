use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A cargo project to run tests against."] # [doc = ""] # [doc = " See [`ProjectBuilder`] or [`Project::from_template`] to get started."] pub struct Project { root : PathBuf , }
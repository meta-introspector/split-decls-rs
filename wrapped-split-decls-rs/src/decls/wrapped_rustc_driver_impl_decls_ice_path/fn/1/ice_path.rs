use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn ice_path () -> & 'static Option < PathBuf > { ice_path_with_config (None) }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Central cargo processor that handles all Cargo.toml operations"] pub struct CargoProcessor { config : SplitDeclsConfig , }
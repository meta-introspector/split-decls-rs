use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Deserialize , Clone)] pub struct BinsConfig { # [serde (flatten)] pub paths : HashMap < String , PathBuf > , }
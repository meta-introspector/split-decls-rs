use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Deserialize)] pub struct RunSearchDetails { pub command : String , pub output_file : Option < PathBuf > , }
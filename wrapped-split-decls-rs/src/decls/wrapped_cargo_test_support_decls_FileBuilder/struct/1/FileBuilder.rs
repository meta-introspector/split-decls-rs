use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (PartialEq , Clone)] struct FileBuilder { path : PathBuf , body : String , executable : bool , }
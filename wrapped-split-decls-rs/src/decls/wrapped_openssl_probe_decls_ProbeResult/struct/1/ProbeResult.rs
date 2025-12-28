use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ProbeResult { pub cert_file : Option < PathBuf > , pub cert_dir : Option < PathBuf > , }
}
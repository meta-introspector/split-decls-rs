use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Represents the generated patches, grouped by their repository URL."] # [doc = " The key is the repository URL (e.g., \"https://github.com/meta-introspector/time-rs\")."] # [doc = " The value is a vector of `PatchEntry` for that repository."] pub type GeneratedPatches = HashMap < String , Vec < PatchEntry > > ;
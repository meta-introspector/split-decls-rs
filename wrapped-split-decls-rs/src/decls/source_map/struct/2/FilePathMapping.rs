use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone)] pub struct FilePathMapping { mapping : Vec < (PathBuf , PathBuf) > , filename_display_for_diagnostics : FileNameDisplayPreference , filename_embeddable_preference : FileNameEmbeddablePreference , }
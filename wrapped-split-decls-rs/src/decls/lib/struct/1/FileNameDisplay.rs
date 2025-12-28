use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct FileNameDisplay < 'a > { inner : & 'a FileName , display_pref : FileNameDisplayPreference , }
}
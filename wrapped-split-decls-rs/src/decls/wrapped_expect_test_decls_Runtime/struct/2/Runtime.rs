use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] struct Runtime { help_printed : bool , per_file : HashMap < & 'static str , FileRuntime > , }
}
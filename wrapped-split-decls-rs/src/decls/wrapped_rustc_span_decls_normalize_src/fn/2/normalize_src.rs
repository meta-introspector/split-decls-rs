use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Normalizes the source code and records the normalizations."] fn normalize_src (src : & mut String) -> Vec < NormalizedPos > { let mut normalized_pos = vec ! [] ; remove_bom (src , & mut normalized_pos) ; normalize_newlines (src , & mut normalized_pos) ; normalized_pos }
}
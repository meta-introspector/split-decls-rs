use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct LintGroups { groups : & 'static [& 'static str] , inside_warnings : bool , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct DependencyTypeStats { string_deps : usize , table_deps : usize , workspace_deps : usize , path_deps : usize , version_deps : usize , }
}
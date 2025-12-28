use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Collect all dependencies from a manifest with metadata"] pub fn collect_dependencies_with_metadata (manifest : & Value) -> HashMap < String , DependencyMetadata > { let mut deps = HashMap :: new () ; for section in ["dependencies" , "dev-dependencies" , "build-dependencies"] { if let Some (section_deps) = manifest . get (section) . and_then (| v | v . as_table ()) { for (name , value) in section_deps { let metadata = extract_dependency_metadata (name , value) ; let key = metadata . name . clone () ; deps . insert (key , metadata) ; } } } deps }
}
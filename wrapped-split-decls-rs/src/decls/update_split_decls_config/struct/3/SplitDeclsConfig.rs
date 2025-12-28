use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default , Serialize , Deserialize)] struct SplitDeclsConfig { # [serde (default)] wrapping : WrappingConfig , # [serde (default)] crate_path_overrides : HashMap < String , String > , pub explicit_crate_path_mappings : Option < HashMap < String , String > > , # [serde (default)] pub workspace_dependency_overrides : Option < HashMap < String , Value > > , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct SplitDeclsConfig { pub patches : HashMap < String , String > , pub string_replacements : HashMap < String , String > , pub custom_prelude_overlay : String , }
}
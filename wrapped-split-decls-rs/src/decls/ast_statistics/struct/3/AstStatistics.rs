use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default , Serialize , Deserialize)] pub struct AstStatistics { pub enum_variants : HashMap < String , VariantStats > , pub constructors : HashMap < String , ConstructorStats > , pub parameters : HashMap < String , ParameterStats > , pub usage_patterns : HashMap < String , PatternStats > , pub type_manifold : TypeManifold , }
}
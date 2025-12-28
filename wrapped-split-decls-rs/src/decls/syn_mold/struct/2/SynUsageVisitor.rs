use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Visitor that extracts syn usage patterns"] pub struct SynUsageVisitor { pub patterns : HashMap < String , Vec < String > > , pub operations : Vec < SynOperation > , pub parse_calls : u32 , pub visit_calls : u32 , pub transform_calls : u32 , pub generation_calls : u32 , }
}
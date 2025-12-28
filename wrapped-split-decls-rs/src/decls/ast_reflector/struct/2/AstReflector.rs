use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default)] pub struct AstReflector { probes : Vec < AstProbe > , collected_data : HashMap < String , Vec < String > > , transformation_count : HashMap < String , usize > , current_file : PathBuf , }
}
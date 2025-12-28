use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct LayerAnalyzer { patterns : HashMap < (CompilerLayer , String) , (usize , Vec < String >) > , emoji_map : HashMap < String , String > , }
}
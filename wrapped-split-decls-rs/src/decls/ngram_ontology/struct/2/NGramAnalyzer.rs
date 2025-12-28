use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct NGramAnalyzer { patterns : HashMap < String , (usize , Vec < String >) > , emoji_map : HashMap < String , String > , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct NGramAnalyzer { patterns : HashMap < String , (usize , Vec < String >) > , emoji_map : HashMap < String , String > , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Result of mold extraction"] pub struct MoldExtraction { pub original_code : syn :: File , pub mold_wrapper : TokenStream , pub static_analysis : SynMold , }
}
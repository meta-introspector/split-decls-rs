use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize)] struct CorePattern { pattern : String , emoji : String , total_frequency : usize , appears_in_ngrams : Vec < usize > , semantic_category : String , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl MoldExtraction { # [doc = " Generate replacement code that uses mold instead of syn"] pub fn generate_replacement (& self) -> TokenStream { let wrapper = & self . mold_wrapper ; let original = & self . original_code ; quote ! { # wrapper # original const _MOLD_ANALYSIS : & str = concat ! ("Complexity: " , stringify ! (# (self . static_analysis . complexity_metrics . total_complexity)) , ", Operations: " , stringify ! (# (self . static_analysis . signatures . len ()))) ; } } }
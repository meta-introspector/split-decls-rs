use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] pub enum QueryType { MaxComplexity , ComplexityAbove (f64) , FrequencyAbove (usize) , PatternMatch (String) , CrossLayerRelation , }
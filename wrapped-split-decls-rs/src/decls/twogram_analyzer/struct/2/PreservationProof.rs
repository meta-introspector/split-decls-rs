use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] struct PreservationProof { total_2grams : usize , preserved_count : usize , preservation_ratio : f64 , compression_mapping : HashMap < String , String > , }
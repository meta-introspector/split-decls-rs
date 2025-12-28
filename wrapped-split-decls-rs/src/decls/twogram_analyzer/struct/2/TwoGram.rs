use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] struct TwoGram { tokens : (String , String) , count : usize , relationship_type : String , preserved_in_next_layer : bool , }
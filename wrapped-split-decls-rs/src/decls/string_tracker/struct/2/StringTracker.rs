use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A struct to track strings from input to output"] # [doc = " Ensures no strings are lost during processing"] # [derive (Debug , Default)] pub struct StringTracker { input_strings : HashMap < u64 , StringRecord > , output_strings : HashMap < u64 , StringRecord > , accounted_for : HashSet < u64 > , }
}
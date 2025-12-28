use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Like Jaro but gives a boost to strings that have a common prefix."] # [doc = ""] # [doc = " ```"] # [doc = " use strsim::jaro_winkler;"] # [doc = ""] # [doc = " assert!((0.866 - jaro_winkler(\"cheeseburger\", \"cheese fries\")).abs() <"] # [doc = "         0.001);"] # [doc = " ```"] pub fn jaro_winkler (a : & str , b : & str) -> f64 { generic_jaro_winkler (& StringWrapper (a) , & StringWrapper (b)) }
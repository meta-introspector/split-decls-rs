use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Case Insensitive wrapper of Ascii strings."] # [derive (Clone , Copy , Debug , Default)] pub struct Ascii < S > (S) ;
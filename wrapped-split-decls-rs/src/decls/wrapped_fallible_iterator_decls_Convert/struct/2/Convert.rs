use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A fallible iterator that wraps a normal iterator over `Result`s."] # [derive (Clone , Debug)] pub struct Convert < I > (I) ;
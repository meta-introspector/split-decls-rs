use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Success variant."] pub struct OkParse < T > (pub T) ;
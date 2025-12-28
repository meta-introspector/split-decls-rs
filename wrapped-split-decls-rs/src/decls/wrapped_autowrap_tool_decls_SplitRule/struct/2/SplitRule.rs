use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Deserialize)] pub struct SplitRule { pub pattern : String , pub wrap_with : Vec < String > , pub imports : Vec < String > , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Default)] pub struct Diff { removed : Vec < bool > , added : Vec < bool > , }
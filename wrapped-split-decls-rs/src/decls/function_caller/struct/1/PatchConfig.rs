use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct PatchConfig { pub patches : HashMap < String , String > , }
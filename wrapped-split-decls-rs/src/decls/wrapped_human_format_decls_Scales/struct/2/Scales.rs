use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Provide a customized scaling scheme for your own modeling."] # [derive (Debug)] pub struct Scales { base : u32 , suffixes : Vec < String > , }
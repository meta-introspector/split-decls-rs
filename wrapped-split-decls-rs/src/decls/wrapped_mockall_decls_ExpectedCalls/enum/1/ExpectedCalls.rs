use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (PartialEq)] # [doc (hidden)] pub enum ExpectedCalls { Satisfied , TooMany , TooFew , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub enum LitError { InvalidSuffix (Symbol) , InvalidIntSuffix (Symbol) , InvalidFloatSuffix (Symbol) , NonDecimalFloat (u32) , IntTooLarge (u32) , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] struct ReplState { system : Output2MacroSystem , variables : HashMap < String , String > , history : Vec < String > , }
}
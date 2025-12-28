use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Macro binding signature - the string of bindings needed for a declaration"] # [derive (Debug , Clone)] pub struct MacroBindingSignature { pub bindings : Vec < String > , pub signature_string : String , pub prime_key : u64 , pub emoji_key : String , pub frequency : u64 , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " `MacroCallId` identifies a particular macro invocation, like"] # [doc = " `println!(\"Hello, {}\", world)`."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct MacroCallId (pub salsa :: Id) ;
}
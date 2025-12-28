use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Represents a replacement of a `snippet`."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct Replacement { # [doc = " Code snippet that gets replaced."] pub snippet : Snippet , # [doc = " The replacement of the snippet."] pub replacement : String , }
}
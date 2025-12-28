use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum HirFileId { FileId (EditionedFileId) , MacroFile (MacroCallId) , }
}
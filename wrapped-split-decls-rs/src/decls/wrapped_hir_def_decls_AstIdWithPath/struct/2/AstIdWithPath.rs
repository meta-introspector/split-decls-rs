use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Helper wrapper for `AstId` with `ModPath`"] # [derive (Clone , Debug , Eq , PartialEq)] struct AstIdWithPath < T : AstIdNode > { ast_id : AstId < T > , path : Interned < ModPath > , }
}
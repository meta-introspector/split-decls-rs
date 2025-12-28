use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , PartialEq , Debug)] pub struct Lit { pub kind : LitKind , pub symbol : Symbol , pub suffix : Option < Symbol > , }
}
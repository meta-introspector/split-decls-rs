use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Literal < S > { pub symbol : Symbol , pub span : S , pub kind : LitKind , pub suffix : Option < Symbol > , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Copy , Clone)] pub (crate) enum DeclOrigin { LetExpr , # [doc = " from `let x = ..`"] LocalDecl { has_else : bool , } , }
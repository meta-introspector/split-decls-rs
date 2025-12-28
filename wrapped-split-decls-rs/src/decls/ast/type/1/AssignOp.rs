use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type AssignOp = Spanned < AssignOpKind > ;
}
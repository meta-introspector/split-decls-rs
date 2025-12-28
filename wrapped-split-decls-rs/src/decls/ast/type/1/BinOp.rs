use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type BinOp = Spanned < BinOpKind > ;
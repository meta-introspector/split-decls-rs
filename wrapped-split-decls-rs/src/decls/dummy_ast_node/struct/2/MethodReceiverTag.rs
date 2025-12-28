use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A tag type used to wrap an `ast::Expr` specifically for method receiver contexts."] pub struct MethodReceiverTag ;
}
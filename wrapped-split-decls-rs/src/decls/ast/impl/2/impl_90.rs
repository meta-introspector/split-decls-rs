use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl UnOp { pub fn as_str (& self) -> & 'static str { match self { UnOp :: Deref => "*" , UnOp :: Not => "!" , UnOp :: Neg => "-" , } } # [doc = " Returns `true` if the unary operator takes its argument by value."] pub fn is_by_value (self) -> bool { matches ! (self , Self :: Neg | Self :: Not) } }
}
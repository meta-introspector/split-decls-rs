use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl GenericArgs { pub fn is_angle_bracketed (& self) -> bool { matches ! (self , AngleBracketed (..)) } pub fn span (& self) -> Span { match self { AngleBracketed (data) => data . span , Parenthesized (data) => data . span , ParenthesizedElided (span) => * span , } } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < ParenthesizedArgs > for Box < GenericArgs > { fn from (val : ParenthesizedArgs) -> Self { Box :: new (GenericArgs :: Parenthesized (val)) } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl KleeneToken { pub fn new (op : KleeneOp , span : Span) -> KleeneToken { KleeneToken { span , op } } }
}
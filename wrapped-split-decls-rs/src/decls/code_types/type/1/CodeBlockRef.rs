use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type CodeBlockRef = Rc < RefCell < CodeBlockNode > > ;
}
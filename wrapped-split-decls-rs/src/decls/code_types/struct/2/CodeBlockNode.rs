use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct CodeBlockNode { pub code_block : CodeBlock , pub parent : Option < Weak < RefCell < CodeBlockNode > > > , }
}
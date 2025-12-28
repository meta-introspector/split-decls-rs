use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn walk_chain (span : Span , to : SyntaxContext) -> Span { HygieneData :: with (| data | data . walk_chain (span , to)) }
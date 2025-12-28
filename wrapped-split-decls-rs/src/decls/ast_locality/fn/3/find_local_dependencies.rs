use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn find_local_dependencies (content : & str) -> Vec < String > { let local_patterns = ["self::" , "super::" , "crate::" , "use crate" , "impl " , "struct " , "fn " , "mod " , "let " , "mut " , "match " , "if " , "for " , "while " , "loop "] ; local_patterns . par_iter () . filter (| pattern | content . contains (* pattern)) . map (| s | s . to_string ()) . collect () }
}
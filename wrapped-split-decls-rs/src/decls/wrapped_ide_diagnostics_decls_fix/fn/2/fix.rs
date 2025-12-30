use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: fix");
fn fix (id : & 'static str , label : & str , source_change : SourceChange , target : TextRange) -> Assist { let mut res = unresolved_fix (id , label , target) ; res . source_change = Some (source_change) ; res }
}
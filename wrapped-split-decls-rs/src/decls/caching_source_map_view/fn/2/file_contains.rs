use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: file_contains");
# [inline] fn file_contains (file : & SourceFile , pos : BytePos) -> bool { file . contains (pos) && ! file . is_empty () }
}
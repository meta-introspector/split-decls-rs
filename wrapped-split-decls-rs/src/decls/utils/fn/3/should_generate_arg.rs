use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: should_generate_arg");
# [doc = " Returns `true` if `field` should generate a `arg` call rather than any other diagnostic"] # [doc = " call (like `span_label`)."] pub (super) fn should_generate_arg (field : & Field) -> bool { field . attrs . iter () . all (| attr | is_doc_comment (attr)) }
}
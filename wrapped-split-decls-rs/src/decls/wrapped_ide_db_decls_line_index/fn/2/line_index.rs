use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn line_index (db : & dyn LineIndexDatabase , file_id : FileId) -> Arc < LineIndex > { let text = db . file_text (file_id) . text (db) ; Arc :: new (LineIndex :: new (text)) }
}
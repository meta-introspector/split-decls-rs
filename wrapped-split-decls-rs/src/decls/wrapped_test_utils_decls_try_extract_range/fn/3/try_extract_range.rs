use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Returns `TextRange` between the first two markers `$0...$0` and the copy"] # [doc = " of `text` without both of these markers."] fn try_extract_range (text : & str) -> Option < (TextRange , String) > { let (start , text) = try_extract_offset (text) ? ; let (end , text) = try_extract_offset (& text) ? ; Some ((TextRange :: new (start , end) , text)) }
}
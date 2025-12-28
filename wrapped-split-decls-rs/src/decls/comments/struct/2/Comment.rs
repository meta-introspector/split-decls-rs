use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone)] pub struct Comment { pub style : CommentStyle , pub lines : Vec < String > , pub pos : BytePos , }
}
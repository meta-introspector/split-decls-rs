use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A `Punct` is a single punctuation character like `+`, `-` or `#`."] # [doc = ""] # [doc = " Multicharacter operators like `+=` are represented as two instances of"] # [doc = " `Punct` with different forms of `Spacing` returned."] # [derive (Clone)] pub struct Punct { ch : char , spacing : Spacing , span : Span , }
}
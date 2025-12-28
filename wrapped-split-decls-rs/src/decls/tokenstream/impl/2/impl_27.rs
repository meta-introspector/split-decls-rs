use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl NodeRange { pub fn new (ParserRange (parser_range) : ParserRange , start_pos : u32) -> NodeRange { assert ! (! parser_range . is_empty ()) ; assert ! (parser_range . start >= start_pos) ; NodeRange ((parser_range . start - start_pos) .. (parser_range . end - start_pos)) } }
}
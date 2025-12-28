use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Given a string like \" ~~~~~~~~~~~~ \", produces a span"] # [doc = " converting that range. The idea is that the string has the same"] # [doc = " length as the input, and we uncover the byte positions. Note"] # [doc = " that this can span lines and so on."] fn span_from_selection (input : & str , selection : & str) -> Span { assert_eq ! (input . len () , selection . len ()) ; let left_index = selection . find ('~') . unwrap () as u32 ; let right_index = selection . rfind ('~') . map_or (left_index , | x | x as u32) ; Span :: with_root_ctxt (BytePos (left_index) , BytePos (right_index + 1)) }
}
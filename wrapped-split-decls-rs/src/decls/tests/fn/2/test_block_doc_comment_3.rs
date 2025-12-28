use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_block_doc_comment_3 () { create_default_session_globals_then (| | { let comment = "\n let a: *i32;\n *a = 5;\n" ; let stripped = beautify_doc_string (Symbol :: intern (comment) , CommentKind :: Block) ; assert_eq ! (stripped . as_str () , "let a: *i32;\n*a = 5;") ; }) }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_block_doc_comment_2 () { create_default_session_globals_then (| | { let comment = "\n * Test\n *  Test\n" ; let stripped = beautify_doc_string (Symbol :: intern (comment) , CommentKind :: Block) ; assert_eq ! (stripped . as_str () , " Test\n  Test") ; }) }
}
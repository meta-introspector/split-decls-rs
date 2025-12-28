use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_line_doc_comment () { create_default_session_globals_then (| | { let stripped = beautify_doc_string (Symbol :: intern (" test") , CommentKind :: Line) ; assert_eq ! (stripped . as_str () , " test") ; let stripped = beautify_doc_string (Symbol :: intern ("! test") , CommentKind :: Line) ; assert_eq ! (stripped . as_str () , "! test") ; let stripped = beautify_doc_string (Symbol :: intern ("test") , CommentKind :: Line) ; assert_eq ! (stripped . as_str () , "test") ; let stripped = beautify_doc_string (Symbol :: intern ("!test") , CommentKind :: Line) ; assert_eq ! (stripped . as_str () , "!test") ; }) }
}
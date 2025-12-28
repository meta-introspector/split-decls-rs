use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_doc_blocks () { create_default_session_globals_then (| | { let stripped = beautify_doc_string (Symbol :: intern (" # Returns\n     *\n     ") , CommentKind :: Block) ; assert_eq ! (stripped . as_str () , " # Returns\n\n") ; let stripped = beautify_doc_string (Symbol :: intern ("\n     * # Returns\n     *\n     ") , CommentKind :: Block ,) ; assert_eq ! (stripped . as_str () , " # Returns\n\n") ; let stripped = beautify_doc_string (Symbol :: intern ("\n *     a\n ") , CommentKind :: Block) ; assert_eq ! (stripped . as_str () , "     a\n") ; }) }
}
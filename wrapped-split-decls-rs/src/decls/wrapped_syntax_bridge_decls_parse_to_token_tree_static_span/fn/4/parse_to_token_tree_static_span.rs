use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Convert a string to a `TokenTree`. The passed span will be used for all spans of the produced subtree."] pub fn parse_to_token_tree_static_span < S > (edition : Edition , span : S , text : & str ,) -> Option < tt :: TopSubtree < S > > where S : Copy + fmt :: Debug , { let lexed = parser :: LexedStr :: new (edition , text) ; if lexed . errors () . next () . is_some () { return None ; } let mut conv = StaticRawConverter { lexed , pos : 0 , span , mode : DocCommentDesugarMode :: ProcMacro , } ; Some (convert_tokens (& mut conv)) }
}
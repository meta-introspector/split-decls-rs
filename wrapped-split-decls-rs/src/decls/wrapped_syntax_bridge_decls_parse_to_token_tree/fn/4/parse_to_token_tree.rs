use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Convert a string to a `TokenTree`. The spans of the subtree will be anchored to the provided"] # [doc = " anchor with the given context."] pub fn parse_to_token_tree < Ctx > (edition : Edition , anchor : SpanAnchor , ctx : Ctx , text : & str ,) -> Option < tt :: TopSubtree < SpanData < Ctx > > > where SpanData < Ctx > : Copy + fmt :: Debug , Ctx : Copy , { let lexed = parser :: LexedStr :: new (edition , text) ; if lexed . errors () . next () . is_some () { return None ; } let mut conv = RawConverter { lexed , anchor , pos : 0 , ctx , mode : DocCommentDesugarMode :: ProcMacro , } ; Some (convert_tokens (& mut conv)) }
}
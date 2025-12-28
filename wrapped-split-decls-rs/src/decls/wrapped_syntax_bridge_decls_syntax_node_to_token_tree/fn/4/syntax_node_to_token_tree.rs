use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Converts a syntax tree to a [`tt::Subtree`] using the provided span map to populate the"] # [doc = " subtree's spans."] pub fn syntax_node_to_token_tree < Ctx , SpanMap > (node : & SyntaxNode , map : SpanMap , span : SpanData < Ctx > , mode : DocCommentDesugarMode ,) -> tt :: TopSubtree < SpanData < Ctx > > where SpanData < Ctx > : Copy + fmt :: Debug , SpanMap : SpanMapper < SpanData < Ctx > > , { let mut c = Converter :: new (node , map , Default :: default () , Default :: default () , span , mode ,) ; convert_tokens (& mut c) }
}
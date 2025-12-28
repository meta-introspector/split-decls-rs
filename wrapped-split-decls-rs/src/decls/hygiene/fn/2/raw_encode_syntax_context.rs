use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn raw_encode_syntax_context (ctxt : SyntaxContext , context : & HygieneEncodeContext , e : & mut impl Encoder ,) { if ! context . serialized_ctxts . lock () . contains (& ctxt) { context . latest_ctxts . lock () . insert (ctxt) ; } ctxt . 0 . encode (e) ; }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Parses the depth used by index(depth) and len(depth)."] fn parse_depth < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , span : Span ,) -> PResult < 'psess , usize > { let Some (tt) = iter . next () else { return Ok (0) } ; let TokenTree :: Token (Token { kind : TokenKind :: Literal (lit) , .. } , _) = tt else { return Err (psess . dcx () . struct_span_err (span , "meta-variable expression depth must be a literal")) ; } ; if let Ok (lit_kind) = LitKind :: from_token_lit (* lit) && let LitKind :: Int (n_u128 , LitIntType :: Unsuffixed) = lit_kind && let Ok (n_usize) = usize :: try_from (n_u128 . get ()) { Ok (n_usize) } else { let msg = "only unsuffixes integer literals are supported in meta-variable expressions" ; Err (psess . dcx () . struct_span_err (span , msg)) } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn parse_token < 'psess , 't > (iter : & mut TokenStreamIter < 't > , psess : & 'psess ParseSess , fallback_span : Span ,) -> PResult < 'psess , & 't Token > { let Some (tt) = iter . next () else { handle_continuation_error (psess , fallback_span , UNSUPPORTED_CONCAT_ELEM_ERR , "unsupported_token_tree_fallback") ? } ; let TokenTree :: Token (token , _) = tt else { handle_continuation_error (psess , tt . span () , UNSUPPORTED_CONCAT_ELEM_ERR , "unsupported_token_tree") ? } ; Ok (token) }
}
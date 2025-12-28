use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Checks if there are any remaining tokens (for example, `${ignore($valid, extra)}`) and create"] # [doc = " a diag with the correct arg count if so."] fn check_trailing_tokens < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , ident : Ident ,) -> PResult < 'psess , () > { if iter . peek () . is_none () { return Ok (()) ; } let (min_or_exact_args , max_args) = match ident . name { sym :: concat => panic ! ("concat takes unlimited tokens but didn't eat them all") , sym :: ignore => (1 , None) , sym :: count => (1 , Some (2)) , sym :: index | sym :: len => (0 , Some (1)) , other => unreachable ! ("unknown MVEs should be rejected earlier (got `{other}`)") , } ; handle_continuation_error (psess , iter_span (iter) . expect ("checked is_none above") , & format ! ("extra tokens in meta-variable expression for `{}`" , ident . name) , "mve_extra_tokens") ? }
}
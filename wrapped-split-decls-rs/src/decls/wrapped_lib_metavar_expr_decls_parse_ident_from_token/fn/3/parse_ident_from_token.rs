use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn parse_ident_from_token < 'psess > (psess : & 'psess ParseSess , token : & Token ,) -> PResult < 'psess , Ident > { if let Some ((elem , is_raw)) = token . ident () { if let IdentIsRaw :: Yes = is_raw { return Err (psess . dcx () . struct_span_err (elem . span , RAW_IDENT_ERR)) ; } return Ok (elem) ; } let token_str = pprust :: token_to_string (token) ; handle_continuation_error (psess , token . span , & format ! ("expected identifier, found `{token_str}`") , "expected_ident" ,) ? }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Expects that the next item is a dollar sign."] fn eat_dollar < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , span : Span ,) -> PResult < 'psess , () > { if try_eat_dollar (iter) { return Ok (()) ; } handle_continuation_error (psess , span , "meta-variables within meta-variable expressions must be referenced using a dollar sign" , "missing_dollar_sign") ? }
}
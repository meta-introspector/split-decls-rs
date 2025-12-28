use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Tries to move the iterator forward returning `true` if there is a dollar sign. If not, then the"] # [doc = " iterator is not modified and the result is `false`."] fn try_eat_dollar (iter : & mut TokenStreamIter < '_ >) -> bool { if let Some (TokenTree :: Token (Token { kind : token :: Dollar , .. } , _)) = iter . peek () { let _ = iter . next () ; return true ; } false }
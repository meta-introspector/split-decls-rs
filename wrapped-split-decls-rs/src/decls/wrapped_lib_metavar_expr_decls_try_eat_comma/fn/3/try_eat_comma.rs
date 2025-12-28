use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Tries to move the iterator forward returning `true` if there is a comma. If not, then the"] # [doc = " iterator is not modified and the result is `false`."] fn try_eat_comma (iter : & mut TokenStreamIter < '_ >) -> bool { if let Some (TokenTree :: Token (Token { kind : token :: Comma , .. } , _ ,)) = iter . peek () { let _ = iter . next () ; return true ; } false }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl StrLit { pub fn as_token_lit (& self) -> token :: Lit { let token_kind = match self . style { StrStyle :: Cooked => token :: Str , StrStyle :: Raw (n) => token :: StrRaw (n) , } ; token :: Lit :: new (token_kind , self . symbol , self . suffix) } }
}
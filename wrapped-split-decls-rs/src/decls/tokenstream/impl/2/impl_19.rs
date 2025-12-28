use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Debug for LazyAttrTokenStream { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "LazyAttrTokenStream({:?})" , self . to_attr_token_stream ()) } }
}
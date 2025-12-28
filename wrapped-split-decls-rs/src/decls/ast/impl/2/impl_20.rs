use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Debug for Label { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "label({:?})" , self . ident) } }
}
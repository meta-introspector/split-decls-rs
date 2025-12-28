use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Display for DiffMode { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { DiffMode :: Error => write ! (f , "Error") , DiffMode :: Source => write ! (f , "Source") , DiffMode :: Forward => write ! (f , "Forward") , DiffMode :: Reverse => write ! (f , "Reverse") , } } }
}
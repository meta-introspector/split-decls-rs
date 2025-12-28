use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for AutoDiffItem { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Differentiating {} -> {}" , self . source , self . target) ? ; write ! (f , " with attributes: {:?}" , self . attrs) } }
}
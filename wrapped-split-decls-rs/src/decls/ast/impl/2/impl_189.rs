use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Debug for ImplPolarity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ImplPolarity :: Positive => "positive" . fmt (f) , ImplPolarity :: Negative (_) => "negative" . fmt (f) , } } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for ParamKindOrd { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ParamKindOrd :: Lifetime => "lifetime" . fmt (f) , ParamKindOrd :: TypeOrConst => "type and const" . fmt (f) , } } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < Wrapped : fmt :: Debug , Tag > fmt :: Debug for AstNodeWrapper < Wrapped , Tag > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AstNodeWrapper") . field ("wrapped" , & self . wrapped) . field ("tag" , & self . tag) . finish () } }
}
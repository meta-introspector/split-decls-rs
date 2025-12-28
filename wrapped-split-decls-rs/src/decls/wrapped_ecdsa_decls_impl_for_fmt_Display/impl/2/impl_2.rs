use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < C > fmt :: Display for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{self:X}") } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < C > fmt :: LowerHex for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for byte in self . to_bytes () { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
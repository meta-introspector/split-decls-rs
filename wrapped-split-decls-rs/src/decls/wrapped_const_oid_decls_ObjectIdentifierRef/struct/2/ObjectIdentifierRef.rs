use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " OID reference type: wrapper for the BER serialization."] # [derive (Eq , Hash , PartialEq , PartialOrd , Ord)] # [repr (transparent)] pub struct ObjectIdentifierRef { # [doc = " BER/DER-serialized bytes (sans ASN.1 tag/length)."] ber : [u8] , }
}
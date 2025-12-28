use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Structure representing the HKDF, capable of HKDF-Expand and HKDF-Extract operations."] # [doc = " Recommendations for the correct usage of the parameters can be found in the"] # [doc = " [crate root](index.html#usage)."] # [doc = ""] # [doc = " This type is generic over HMAC implementation. Most users should use"] # [doc = " [`Hkdf`] or [`SimpleHkdf`] type aliases."] # [derive (Clone , Debug)] pub struct GenericHkdf < H : HmacImpl > { hmac : H , }
}
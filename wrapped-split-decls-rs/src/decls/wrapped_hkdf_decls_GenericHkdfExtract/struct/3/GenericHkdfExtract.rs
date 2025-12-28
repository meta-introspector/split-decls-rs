use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Structure representing the streaming context of an HKDF-Extract operation."] # [doc = ""] # [doc = " This type is generic over HMAC implementation. Most users should use"] # [doc = " [`HkdfExtract`] or [`SimpleHkdfExtract`] type aliases."] # [derive (Clone , Debug)] pub struct GenericHkdfExtract < H : HmacImpl > { hmac : H , }
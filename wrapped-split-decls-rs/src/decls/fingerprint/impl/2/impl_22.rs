use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder > Decodable < D > for PackedFingerprint { # [inline] fn decode (d : & mut D) -> Self { Self (Fingerprint :: decode (d)) } }
}
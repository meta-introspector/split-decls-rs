use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : Decoder > Decodable < D > for Fingerprint { # [inline] fn decode (d : & mut D) -> Self { Fingerprint :: from_le_bytes (d . read_raw_bytes (16) . try_into () . unwrap ()) } }
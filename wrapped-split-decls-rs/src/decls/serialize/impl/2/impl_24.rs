use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : Decoder > Decodable < D > for NonZero < u32 > { fn decode (d : & mut D) -> Self { NonZero :: new (d . read_u32 ()) . unwrap () } }
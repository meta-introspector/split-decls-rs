use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : Decoder > Decodable < D > for String { fn decode (d : & mut D) -> String { d . read_str () . to_owned () } }
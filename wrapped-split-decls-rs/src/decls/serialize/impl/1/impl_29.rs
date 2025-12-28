use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : Decoder > Decodable < D > for () { fn decode (_ : & mut D) { } }
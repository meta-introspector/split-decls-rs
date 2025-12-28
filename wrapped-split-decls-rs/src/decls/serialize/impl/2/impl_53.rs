use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : Decoder > Decodable < D > for path :: PathBuf { fn decode (d : & mut D) -> path :: PathBuf { let bytes : String = Decodable :: decode (d) ; path :: PathBuf :: from (bytes) } }
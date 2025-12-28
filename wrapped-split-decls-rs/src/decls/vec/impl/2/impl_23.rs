use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "nightly")] impl < D : Decoder , I : Idx , T : Decodable < D > > Decodable < D > for IndexVec < I , T > { fn decode (d : & mut D) -> Self { IndexVec :: from_raw (Vec :: < T > :: decode (d)) } }
}
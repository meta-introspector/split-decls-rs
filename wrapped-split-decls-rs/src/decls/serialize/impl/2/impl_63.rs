use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : Decoder , A : Array < Item : Decodable < D > > > Decodable < D > for SmallVec < A > { fn decode (d : & mut D) -> SmallVec < A > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
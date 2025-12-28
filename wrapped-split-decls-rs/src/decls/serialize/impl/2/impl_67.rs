use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder , T : Decodable < D > > Decodable < D > for VecDeque < T > { fn decode (d : & mut D) -> VecDeque < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
}
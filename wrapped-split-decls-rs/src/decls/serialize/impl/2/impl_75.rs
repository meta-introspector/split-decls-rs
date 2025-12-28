use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < D : Decoder , T , S > Decodable < D > for HashSet < T , S > where T : Decodable < D > + Hash + Eq , S : BuildHasher + Default , { fn decode (d : & mut D) -> HashSet < T , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
}
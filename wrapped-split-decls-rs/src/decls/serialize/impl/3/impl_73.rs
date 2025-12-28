use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : Decoder , K , V , S > Decodable < D > for HashMap < K , V , S > where K : Decodable < D > + Hash + Eq , V : Decodable < D > , S : BuildHasher + Default , { fn decode (d : & mut D) -> HashMap < K , V , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | (Decodable :: decode (d) , Decodable :: decode (d))) . collect () } }
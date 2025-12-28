use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < D : Decoder , T : Decodable < D > > Decodable < D > for Arc < [T] > { fn decode (d : & mut D) -> Arc < [T] > { let vec : Vec < T > = Decodable :: decode (d) ; vec . into () } }
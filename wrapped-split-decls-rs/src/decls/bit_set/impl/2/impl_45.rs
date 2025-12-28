use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Idx > fmt :: Debug for ChunkedBitSet < T > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { w . debug_list () . entries (self . iter ()) . finish () } }
}
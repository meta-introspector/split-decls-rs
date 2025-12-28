use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Idx > fmt :: Debug for MixedBitSet < T > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MixedBitSet :: Small (set) => set . fmt (w) , MixedBitSet :: Large (set) => set . fmt (w) , } } }
}
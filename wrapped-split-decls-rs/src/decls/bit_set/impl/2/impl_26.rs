use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Idx > fmt :: Debug for DenseBitSet < T > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { w . debug_list () . entries (self . iter ()) . finish () } }
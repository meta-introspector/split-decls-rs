use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T : fmt :: Debug > fmt :: Debug for IndexVec < I , T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . raw , fmt) } }
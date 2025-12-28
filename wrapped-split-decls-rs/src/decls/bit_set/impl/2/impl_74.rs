use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: fmt :: Debug for FiniteBitSet < u32 > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:032b}" , self . 0) } }
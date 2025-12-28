use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for Hash128 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }
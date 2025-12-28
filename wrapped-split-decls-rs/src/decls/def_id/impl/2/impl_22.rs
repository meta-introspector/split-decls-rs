use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: LowerHex for StableCrateId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . 0 , f) } }
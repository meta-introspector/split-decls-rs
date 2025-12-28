use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl core :: fmt :: Debug for Digest { # [inline] fn fmt (& self , formatter : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: LowerHex :: fmt (self , formatter) } }
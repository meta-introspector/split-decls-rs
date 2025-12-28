use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : fmt :: Debug > fmt :: Debug for UniCase < S > { # [inline] fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (inner ! (self . 0) , fmt) } }
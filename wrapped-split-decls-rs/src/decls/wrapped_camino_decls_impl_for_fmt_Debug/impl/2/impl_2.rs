use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for Utf8PrefixComponent < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
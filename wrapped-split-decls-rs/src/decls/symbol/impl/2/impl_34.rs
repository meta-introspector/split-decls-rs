use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for Symbol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_str () , f) } }
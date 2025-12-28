use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Display for Symbol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
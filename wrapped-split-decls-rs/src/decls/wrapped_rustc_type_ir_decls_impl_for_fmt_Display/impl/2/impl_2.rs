use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Display for ClosureKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_str () . fmt (f) } }
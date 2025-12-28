use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : Display + Internable + ? Sized > Display for Interned < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self . arc) . fmt (f) } }
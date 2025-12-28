use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > fmt :: Debug for Layout < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . 0 . fmt (f) } }
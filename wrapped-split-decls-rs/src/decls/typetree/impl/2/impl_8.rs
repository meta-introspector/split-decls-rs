use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Display for Type { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < Self as fmt :: Debug > :: fmt (self , f) } }
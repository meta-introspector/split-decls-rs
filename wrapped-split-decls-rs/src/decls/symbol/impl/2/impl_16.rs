use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for Ident { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , f) ? ; fmt :: Debug :: fmt (& self . span . ctxt () , f) } }
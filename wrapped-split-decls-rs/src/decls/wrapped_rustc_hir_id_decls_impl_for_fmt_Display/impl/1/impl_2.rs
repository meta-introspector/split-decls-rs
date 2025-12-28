use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Display for HirId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{self:?}") } }
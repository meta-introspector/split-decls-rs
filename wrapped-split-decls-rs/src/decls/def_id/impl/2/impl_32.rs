use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for DefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* DEF_ID_DEBUG) (* self , f) } }
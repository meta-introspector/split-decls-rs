use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for Endian { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . as_str ()) } }
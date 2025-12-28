use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for ByteSymbol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_byte_str () , f) } }
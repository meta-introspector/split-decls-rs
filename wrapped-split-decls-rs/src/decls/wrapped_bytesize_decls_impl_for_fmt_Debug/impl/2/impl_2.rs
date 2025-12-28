use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for ByteSize { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} ({} bytes)" , self , self . 0) } }
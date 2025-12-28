use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for Size { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Size({} bytes)" , self . bytes ()) } }
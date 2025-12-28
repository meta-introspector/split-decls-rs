use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Display for OutOfRange { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "out of range") } }
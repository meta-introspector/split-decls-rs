use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for AlignFromBytesError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , f) } }
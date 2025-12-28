use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Display for BaseNString { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self) } }
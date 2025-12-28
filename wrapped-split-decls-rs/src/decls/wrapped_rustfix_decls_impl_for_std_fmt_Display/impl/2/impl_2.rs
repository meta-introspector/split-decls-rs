use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: fmt :: Display for LineRange { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}-{}" , self . start , self . end) } }
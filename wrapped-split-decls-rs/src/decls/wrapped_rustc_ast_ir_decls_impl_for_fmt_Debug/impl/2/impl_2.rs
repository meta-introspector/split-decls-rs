use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for FloatTy { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . name_str ()) } }
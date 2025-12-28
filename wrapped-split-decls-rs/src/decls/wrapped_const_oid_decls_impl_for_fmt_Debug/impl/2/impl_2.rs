use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for ObjectIdentifierRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ObjectIdentifierRef({self})") } }
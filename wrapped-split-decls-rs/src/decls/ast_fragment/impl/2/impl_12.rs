use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: fmt :: Display for AstFragment { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}" , self) } }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: fmt :: Debug for SalsaAttr { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (fmt , "{:?}" , self . name) } }
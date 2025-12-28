use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl std :: fmt :: Debug for ReprFlags { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { bitflags :: parser :: to_writer (self , f) } }
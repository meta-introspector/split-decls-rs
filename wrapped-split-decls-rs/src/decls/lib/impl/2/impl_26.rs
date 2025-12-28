use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl IntegerType { pub fn is_signed (& self) -> bool { match self { IntegerType :: Pointer (b) => * b , IntegerType :: Fixed (_ , b) => * b , } } }
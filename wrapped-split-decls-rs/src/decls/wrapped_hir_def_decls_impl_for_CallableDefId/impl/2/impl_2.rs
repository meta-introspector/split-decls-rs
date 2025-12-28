use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl CallableDefId { pub fn krate (self , db : & dyn DefDatabase) -> Crate { match self { CallableDefId :: FunctionId (f) => f . krate (db) , CallableDefId :: StructId (s) => s . krate (db) , CallableDefId :: EnumVariantId (e) => e . krate (db) , } } }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl HasVisibility for AssocItem { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { match self { AssocItem :: Function (f) => f . visibility (db) , AssocItem :: Const (c) => c . visibility (db) , AssocItem :: TypeAlias (t) => t . visibility (db) , } } }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Copy)] pub enum SynLangPatterns { File , Item , ItemFn , ItemStruct , Expr , ExprCall , Type , Pat , Ident , Block , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub enum AstNodeType { Function , Struct , Enum , Impl , Trait , Module , Use , Const , Static , Type , Macro , Expr , Stmt , Pat , All , }
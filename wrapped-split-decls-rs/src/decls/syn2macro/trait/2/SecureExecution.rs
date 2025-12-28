use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Security and ACL trait for AST operations"] pub trait SecureExecution { fn check_permission (& self , operation : & AstOperation) -> bool ; fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T ; }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl SecurityContext { pub fn check_permission (& self , operation : & AstOperation) -> bool { match self { SecurityContext :: Default => true , SecurityContext :: Strict (allowed) => { allowed . iter () . any (| op | { std :: mem :: discriminant (op) == std :: mem :: discriminant (operation) }) } } } }
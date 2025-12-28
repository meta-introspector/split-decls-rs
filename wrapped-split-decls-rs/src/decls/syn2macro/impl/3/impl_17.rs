use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl SecureExecution for StrictSecurity { fn check_permission (& self , operation : & AstOperation) -> bool { self . allowed_operations . iter () . any (| op | { std :: mem :: discriminant (op) == std :: mem :: discriminant (operation) }) } fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T { Ok (f ()) } }
}
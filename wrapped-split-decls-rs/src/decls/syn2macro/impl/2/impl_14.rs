use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl SecureExecution for DefaultSecurity { fn check_permission (& self , _operation : & AstOperation) -> bool { true } fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T { Ok (f ()) } }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Strict security implementation with ACL"] pub struct StrictSecurity { allowed_operations : Vec < AstOperation > , }
}
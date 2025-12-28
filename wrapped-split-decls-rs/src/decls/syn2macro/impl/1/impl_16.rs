use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl StrictSecurity { pub fn new (allowed_operations : Vec < AstOperation >) -> Self { Self { allowed_operations } } }
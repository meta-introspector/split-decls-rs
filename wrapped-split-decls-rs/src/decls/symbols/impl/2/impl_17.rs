use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Errors { fn error (& mut self , span : Span , message : String) { self . list . push (syn :: Error :: new (span , message)) ; } }
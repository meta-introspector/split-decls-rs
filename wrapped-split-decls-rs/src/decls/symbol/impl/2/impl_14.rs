use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq for Ident { # [inline] fn eq (& self , rhs : & Self) -> bool { self . name == rhs . name && self . span . eq_ctxt (rhs . span) } }
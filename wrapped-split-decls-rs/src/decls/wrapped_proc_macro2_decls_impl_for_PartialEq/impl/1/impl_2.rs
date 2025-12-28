use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq for Ident { fn eq (& self , other : & Ident) -> bool { self . inner == other . inner } }
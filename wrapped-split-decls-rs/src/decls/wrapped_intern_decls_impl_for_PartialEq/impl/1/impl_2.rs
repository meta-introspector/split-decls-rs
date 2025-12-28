use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq for Interned < str > { fn eq (& self , other : & Self) -> bool { Arc :: ptr_eq (& self . arc , & other . arc) } }
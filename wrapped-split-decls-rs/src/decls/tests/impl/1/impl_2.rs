use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq for S { fn eq (& self , _other : & Self) -> bool { panic ! ("shouldn't be called") ; } }
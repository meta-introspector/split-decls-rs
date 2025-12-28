use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < W : fmt :: Write > DemangleWrite for W { fn write_string (& mut self , s : & str) -> fmt :: Result { fmt :: Write :: write_str (self , s) } }
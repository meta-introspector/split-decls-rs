use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl fmt :: Debug for RootDatabase { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RootDatabase") . finish () } }
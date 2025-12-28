use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Display for Bom { # [doc = " Formats the BOM type as a `String`."] fn fmt (& self , formatter : & mut Formatter) -> fmt :: Result { write ! (formatter , "{}" , AsRef ::< str >:: as_ref (self)) } }
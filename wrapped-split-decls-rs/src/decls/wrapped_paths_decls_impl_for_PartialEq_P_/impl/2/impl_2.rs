use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < P : AsRef < Path > + ? Sized > PartialEq < P > for AbsPath { fn eq (& self , other : & P) -> bool { self . 0 . as_std_path () == other . as_ref () } }
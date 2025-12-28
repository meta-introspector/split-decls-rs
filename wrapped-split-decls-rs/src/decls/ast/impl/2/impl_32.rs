use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq < Symbol > for PathSegment { # [inline] fn eq (& self , name : & Symbol) -> bool { self . args . is_none () && self . ident . name == * name } }
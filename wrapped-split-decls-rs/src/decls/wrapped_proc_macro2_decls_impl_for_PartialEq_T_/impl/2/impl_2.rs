use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > PartialEq < T > for Ident where T : ? Sized + AsRef < str > , { fn eq (& self , other : & T) -> bool { self . inner == other } }
}
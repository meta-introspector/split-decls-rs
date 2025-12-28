use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > FileNameDisplay < 'a > { pub fn to_string_lossy (& self) -> Cow < 'a , str > { match self . inner { FileName :: Real (inner) => inner . to_string_lossy (self . display_pref) , _ => Cow :: from (self . to_string ()) , } } }
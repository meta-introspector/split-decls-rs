use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl CfgOverrides { pub fn len (& self) -> usize { self . global . len () + self . selective . values () . map (| it | it . len ()) . sum :: < usize > () } pub fn apply (& self , cfg_options : & mut cfg :: CfgOptions , name : & str) { if ! self . global . is_empty () { cfg_options . apply_diff (self . global . clone ()) ; } if let Some (diff) = self . selective . get (name) { cfg_options . apply_diff (diff . clone ()) ; } } }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn entry_point_type (attrs : & [impl AttributeExt] , at_root : bool , name : Option < Symbol > ,) -> EntryPointType { if attr :: contains_name (attrs , sym :: rustc_main) { EntryPointType :: RustcMainAttr } else if let Some (name) = name && name == sym :: main { if at_root { EntryPointType :: MainNamed } else { EntryPointType :: OtherMain } } else { EntryPointType :: None } }
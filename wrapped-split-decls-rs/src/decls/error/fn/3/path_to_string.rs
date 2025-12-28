use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Helper function for printing `syn::Path` - doesn't handle arguments in paths and these are"] # [doc = " unlikely to come up much in use of the macro."] fn path_to_string (path : & syn :: Path) -> String { let mut out = String :: new () ; for (i , segment) in path . segments . iter () . enumerate () { if i > 0 || path . leading_colon . is_some () { out . push_str ("::") ; } out . push_str (& segment . ident . to_string ()) ; } out }
}
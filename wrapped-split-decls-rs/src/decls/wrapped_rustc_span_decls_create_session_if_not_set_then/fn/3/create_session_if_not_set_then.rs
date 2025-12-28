use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " No source map."] pub fn create_session_if_not_set_then < R , F > (edition : Edition , f : F) -> R where F : FnOnce (& SessionGlobals) -> R , { if ! SESSION_GLOBALS . is_set () { let session_globals = SessionGlobals :: new (edition , & [] , None) ; SESSION_GLOBALS . set (& session_globals , | | SESSION_GLOBALS . with (f)) } else { SESSION_GLOBALS . with (f) } }
}
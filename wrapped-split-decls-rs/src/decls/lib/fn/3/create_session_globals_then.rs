use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn create_session_globals_then < R > (edition : Edition , extra_symbols : & [& 'static str] , sm_inputs : Option < SourceMapInputs > , f : impl FnOnce () -> R ,) -> R { assert ! (! SESSION_GLOBALS . is_set () , "SESSION_GLOBALS should never be overwritten! \
         Use another thread if you need another SessionGlobals") ; let session_globals = SessionGlobals :: new (edition , extra_symbols , sm_inputs) ; SESSION_GLOBALS . set (& session_globals , f) }
}
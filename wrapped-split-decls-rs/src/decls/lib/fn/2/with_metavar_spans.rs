use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] pub fn with_metavar_spans < R > (f : impl FnOnce (& MetavarSpansMap) -> R) -> R { with_session_globals (| session_globals | f (& session_globals . metavar_spans)) }
}
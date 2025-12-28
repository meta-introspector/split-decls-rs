use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn get_source_map () -> Option < Arc < SourceMap > > { with_session_globals (| session_globals | session_globals . source_map . clone ()) }
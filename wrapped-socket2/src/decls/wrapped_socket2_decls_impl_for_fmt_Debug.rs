use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(target_os = "redox"))]
impl<'name, 'bufs, 'control> fmt::Debug for MsgHdrMut<'name, 'bufs, 'control> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        "MsgHdrMut".fmt(fmt)
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Debug for ThreadLocalIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ThreadLocalIndex").finish()
    }
}

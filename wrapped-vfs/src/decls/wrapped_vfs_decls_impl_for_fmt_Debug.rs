use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Debug for Vfs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vfs")
            .field("n_files", &self.data.len())
            .finish()
    }
}

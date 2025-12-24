use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

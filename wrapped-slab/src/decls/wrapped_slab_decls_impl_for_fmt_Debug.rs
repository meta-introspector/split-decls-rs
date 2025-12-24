use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> fmt::Debug for Drain<'_, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Drain").finish()
    }
}

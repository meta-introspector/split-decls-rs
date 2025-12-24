use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S> fmt::Display for Punct<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.char, f)
    }
}

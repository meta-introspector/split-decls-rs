use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: AsRef<str>> PartialOrd for UniCase<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

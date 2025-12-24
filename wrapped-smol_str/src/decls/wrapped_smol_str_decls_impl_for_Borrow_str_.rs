use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Borrow<str> for SmolStr {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

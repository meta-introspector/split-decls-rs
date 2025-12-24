use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S: AsRef<str>> AsRef<str> for UniCase<S> {
    #[inline]
    fn as_ref(&self) -> &str {
        inner!(self.0).as_ref()
    }
}

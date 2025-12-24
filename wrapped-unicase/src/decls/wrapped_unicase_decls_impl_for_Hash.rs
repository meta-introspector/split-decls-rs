use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S: AsRef<str>> Hash for UniCase<S> {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self.0 {
            Encoding::Ascii(ref s) => s.hash(hasher),
            Encoding::Unicode(ref s) => s.hash(hasher),
        }
    }
}

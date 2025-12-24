use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> Borrow<[T]> for ThinVec<T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

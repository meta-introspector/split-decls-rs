use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Clone> Clone for IntoIter<T> {
    #[allow(clippy::into_iter_on_ref)]
    fn clone(&self) -> Self {
        self.as_slice().into_iter().cloned().collect::<ThinVec<_>>().into_iter()
    }
}

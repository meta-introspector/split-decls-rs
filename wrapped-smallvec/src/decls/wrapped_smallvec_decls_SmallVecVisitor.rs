use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "serde")]
struct SmallVecVisitor<T, const N: usize> {
    phantom: PhantomData<T>,
}

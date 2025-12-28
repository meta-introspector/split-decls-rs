use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > ExactSizeIterator for IntoIter < T , N > { }
}
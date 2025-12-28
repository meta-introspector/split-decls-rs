use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > core :: iter :: FusedIterator for IntoIter < T , N > { }
}
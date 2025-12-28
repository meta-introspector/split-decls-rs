use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T , const N : usize > ExactSizeIterator for IntoIter < T , N > { }
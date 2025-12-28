use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn with_elements_chunked (elements : & [usize] , domain_size : usize) -> ChunkedBitSet < usize > { let mut s = ChunkedBitSet :: new_empty (domain_size) ; for & e in elements { assert ! (s . insert (e)) ; } s }
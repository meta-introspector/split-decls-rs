use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl RevparseMode { is_bit_set ! (is_no_single , RevparseMode :: SINGLE) ; is_bit_set ! (is_range , RevparseMode :: RANGE) ; is_bit_set ! (is_merge_base , RevparseMode :: MERGE_BASE) ; }
}
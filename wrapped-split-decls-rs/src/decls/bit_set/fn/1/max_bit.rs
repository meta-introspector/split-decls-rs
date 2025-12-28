use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn max_bit (word : Word) -> usize { WORD_BITS - 1 - word . leading_zeros () as usize }
}
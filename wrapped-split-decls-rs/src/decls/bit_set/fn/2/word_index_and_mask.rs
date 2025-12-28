use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn word_index_and_mask < T : Idx > (elem : T) -> (usize , Word) { let elem = elem . index () ; let word_index = elem / WORD_BITS ; let mask = 1 << (elem % WORD_BITS) ; (word_index , mask) }
}
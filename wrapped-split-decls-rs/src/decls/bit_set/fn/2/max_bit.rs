use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: max_bit");
# [inline] fn max_bit (word : Word) -> usize { WORD_BITS - 1 - word . leading_zeros () as usize }
}
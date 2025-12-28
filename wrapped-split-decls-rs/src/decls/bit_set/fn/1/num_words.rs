use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn num_words < T : Idx > (domain_size : T) -> usize { domain_size . index () . div_ceil (WORD_BITS) }
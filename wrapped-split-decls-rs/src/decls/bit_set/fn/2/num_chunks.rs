use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn num_chunks < T : Idx > (domain_size : T) -> usize { assert ! (domain_size . index () > 0) ; domain_size . index () . div_ceil (CHUNK_BITS) }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [bench] fn bench_remove (b : & mut Bencher) { let mut bs = DenseBitSet :: new_filled (99999usize) ; b . iter (| | { black_box (bs . remove (black_box (100u32))) ; }) ; }
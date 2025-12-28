use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [bench] fn bench_insert (b : & mut Bencher) { let mut bs = DenseBitSet :: new_filled (99999usize) ; b . iter (| | { black_box (bs . insert (black_box (100u32))) ; }) ; }
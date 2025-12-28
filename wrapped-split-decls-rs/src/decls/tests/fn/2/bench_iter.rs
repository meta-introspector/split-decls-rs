use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [bench] fn bench_iter (b : & mut Bencher) { let bs = DenseBitSet :: new_filled (99999usize) ; b . iter (| | { bs . iter () . map (| b : usize | black_box (b)) . for_each (drop) ; }) ; }
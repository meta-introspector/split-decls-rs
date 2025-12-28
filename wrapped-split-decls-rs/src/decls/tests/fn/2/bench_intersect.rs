use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [bench] fn bench_intersect (b : & mut Bencher) { let mut ba : DenseBitSet < u32 > = DenseBitSet :: new_filled (99999usize) ; let bb = DenseBitSet :: new_filled (99999usize) ; b . iter (| | { ba . intersect (black_box (& bb)) ; }) ; }
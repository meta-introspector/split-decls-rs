use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " This data structure optimizes for cases where the stored bits in each row"] # [doc = " are expected to be highly contiguous (long ranges of 1s or 0s), in contrast"] # [doc = " to BitMatrix and SparseBitMatrix which are optimized for"] # [doc = " \"random\"/non-contiguous bits and cheap(er) point queries at the expense of"] # [doc = " memory usage."] # [derive (Clone)] pub struct SparseIntervalMatrix < R , C > where R : Idx , C : Idx , { rows : IndexVec < R , IntervalSet < C > > , column_size : usize , }
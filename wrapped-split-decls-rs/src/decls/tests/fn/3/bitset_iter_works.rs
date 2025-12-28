use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn bitset_iter_works () { let mut bitset : DenseBitSet < usize > = DenseBitSet :: new_empty (100) ; bitset . insert (1) ; bitset . insert (10) ; bitset . insert (19) ; bitset . insert (62) ; bitset . insert (63) ; bitset . insert (64) ; bitset . insert (65) ; bitset . insert (66) ; bitset . insert (99) ; assert_eq ! (bitset . iter () . collect ::< Vec < _ >> () , [1 , 10 , 19 , 62 , 63 , 64 , 65 , 66 , 99]) ; }
}
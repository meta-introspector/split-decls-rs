use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: bitset_iter_works_2");
# [test] fn bitset_iter_works_2 () { let mut bitset : DenseBitSet < usize > = DenseBitSet :: new_empty (320) ; bitset . insert (0) ; bitset . insert (127) ; bitset . insert (191) ; bitset . insert (255) ; bitset . insert (319) ; assert_eq ! (bitset . iter () . collect ::< Vec < _ >> () , [0 , 127 , 191 , 255 , 319]) ; }
}
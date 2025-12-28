use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn dense_contains_any () { let mut set : DenseBitSet < usize > = DenseBitSet :: new_empty (300) ; assert ! (! set . contains_any (0 .. 300)) ; set . insert_range (10 .. 20) ; set . insert_range (60 .. 70) ; set . insert_range (150 ..= 250) ; assert ! (set . contains_any (0 .. 30)) ; assert ! (set . contains_any (5 .. 100)) ; assert ! (set . contains_any (250 .. 255)) ; assert ! (! set . contains_any (20 .. 59)) ; assert ! (! set . contains_any (256 .. 290)) ; set . insert (22) ; assert ! (set . contains_any (20 .. 59)) ; }
}
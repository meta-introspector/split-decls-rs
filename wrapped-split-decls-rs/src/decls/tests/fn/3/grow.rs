use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn grow () { let mut set : GrowableBitSet < usize > = GrowableBitSet :: with_capacity (65) ; for index in 0 .. 65 { assert ! (set . insert (index)) ; assert ! (! set . insert (index)) ; } set . ensure (128) ; for index in 0 .. 65 { assert ! (set . contains (index)) ; } for index in 65 .. 128 { assert ! (! set . contains (index)) ; } for index in 65 .. 128 { assert ! (set . insert (index)) ; assert ! (! set . insert (index)) ; } }
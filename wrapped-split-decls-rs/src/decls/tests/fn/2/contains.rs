use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn contains () { let mut set = IntervalSet :: new (300) ; set . insert (0u32) ; assert ! (set . contains (0)) ; set . insert_range (0 .. 10) ; assert ! (set . contains (9)) ; assert ! (! set . contains (10)) ; set . insert_range (10 .. 11) ; assert ! (set . contains (10)) ; }
}
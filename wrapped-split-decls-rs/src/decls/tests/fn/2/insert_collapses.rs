use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn insert_collapses () { let mut set = IntervalSet :: < u32 > :: new (10000) ; set . insert_range (9831 ..= 9837) ; set . insert_range (43 ..= 9830) ; assert_eq ! (set . iter_intervals () . collect ::< Vec < _ >> () , [43 .. 9838]) ; }
}
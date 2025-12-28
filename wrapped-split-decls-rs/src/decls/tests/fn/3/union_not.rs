use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn union_not () { let mut a = DenseBitSet :: < usize > :: new_empty (100) ; let mut b = DenseBitSet :: < usize > :: new_empty (100) ; a . insert (3) ; a . insert (5) ; a . insert (80) ; a . insert (81) ; b . insert (5) ; b . insert (7) ; b . insert (63) ; b . insert (81) ; b . insert (90) ; a . union_not (& b) ; assert_eq ! (a . iter () . collect ::< Vec < _ >> () , (0usize .. 100) . filter (|& x | ! matches ! (x , 7 | 63 | 90)) . collect ::< Vec < _ >> () ,) ; }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn bitset_clone_from () { let mut a : DenseBitSet < usize > = DenseBitSet :: new_empty (10) ; a . insert (4) ; a . insert (7) ; a . insert (9) ; let mut b = DenseBitSet :: new_empty (2) ; b . clone_from (& a) ; assert_eq ! (b . domain_size () , 10) ; assert_eq ! (b . iter () . collect ::< Vec < _ >> () , [4 , 7 , 9]) ; b . clone_from (& DenseBitSet :: new_empty (40)) ; assert_eq ! (b . domain_size () , 40) ; assert_eq ! (b . iter () . collect ::< Vec < _ >> () , []) ; }
}
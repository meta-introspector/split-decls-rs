use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test_N5_Cmp_N3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N5CmpN3 = < A as Cmp < B > > :: Output ; assert_eq ! (< N5CmpN3 as Ord >:: to_ordering () , Ordering :: Less) ; }
}
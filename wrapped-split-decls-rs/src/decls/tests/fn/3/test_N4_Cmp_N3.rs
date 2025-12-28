use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Cmp_N3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N4CmpN3 = < A as Cmp < B > > :: Output ; assert_eq ! (< N4CmpN3 as Ord >:: to_ordering () , Ordering :: Less) ; }
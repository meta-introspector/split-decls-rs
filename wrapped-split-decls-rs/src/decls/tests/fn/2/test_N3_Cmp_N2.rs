use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Cmp_N2 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N3CmpN2 = < A as Cmp < B > > :: Output ; assert_eq ! (< N3CmpN2 as Ord >:: to_ordering () , Ordering :: Less) ; }
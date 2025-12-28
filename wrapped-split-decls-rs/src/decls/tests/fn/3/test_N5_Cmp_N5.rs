use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_Cmp_N5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type N5CmpN5 = < A as Cmp < B > > :: Output ; assert_eq ! (< N5CmpN5 as Ord >:: to_ordering () , Ordering :: Equal) ; }
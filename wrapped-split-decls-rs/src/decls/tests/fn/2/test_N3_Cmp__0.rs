use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Cmp__0 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = Z0 ; # [allow (non_camel_case_types)] type N3Cmp_0 = < A as Cmp < B > > :: Output ; assert_eq ! (< N3Cmp_0 as Ord >:: to_ordering () , Ordering :: Less) ; }
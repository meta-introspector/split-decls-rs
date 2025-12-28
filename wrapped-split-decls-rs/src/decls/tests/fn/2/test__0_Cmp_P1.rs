use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Cmp_P1 () { type A = Z0 ; type B = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type _0CmpP1 = < A as Cmp < B > > :: Output ; assert_eq ! (< _0CmpP1 as Ord >:: to_ordering () , Ordering :: Less) ; }
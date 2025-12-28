use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Cmp_P4 () { type A = Z0 ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type _0CmpP4 = < A as Cmp < B > > :: Output ; assert_eq ! (< _0CmpP4 as Ord >:: to_ordering () , Ordering :: Less) ; }
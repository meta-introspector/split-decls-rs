use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Cmp_N1 () { type A = Z0 ; type B = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type _0CmpN1 = < A as Cmp < B > > :: Output ; assert_eq ! (< _0CmpN1 as Ord >:: to_ordering () , Ordering :: Greater) ; }
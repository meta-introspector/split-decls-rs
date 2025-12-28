use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_2_Cmp_0 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UTerm ; # [allow (non_camel_case_types)] type U2CmpU0 = < A as Cmp < B > > :: Output ; assert_eq ! (< U2CmpU0 as Ord >:: to_ordering () , Ordering :: Greater) ; }
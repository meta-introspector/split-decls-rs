use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_Cmp_0 () { type A = UInt < UTerm , B1 > ; type B = UTerm ; # [allow (non_camel_case_types)] type U1CmpU0 = < A as Cmp < B > > :: Output ; assert_eq ! (< U1CmpU0 as Ord >:: to_ordering () , Ordering :: Greater) ; }
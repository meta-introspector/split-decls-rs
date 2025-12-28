use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_5_Cmp_4 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U5CmpU4 = < A as Cmp < B > > :: Output ; assert_eq ! (< U5CmpU4 as Ord >:: to_ordering () , Ordering :: Greater) ; }
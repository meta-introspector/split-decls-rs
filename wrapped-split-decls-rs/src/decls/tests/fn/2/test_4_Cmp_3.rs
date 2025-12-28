use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_4_Cmp_3 () { type A = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U4CmpU3 = < A as Cmp < B > > :: Output ; assert_eq ! (< U4CmpU3 as Ord >:: to_ordering () , Ordering :: Greater) ; }
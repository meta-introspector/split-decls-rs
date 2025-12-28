use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_1_Cmp_3 () { type A = UInt < UTerm , B1 > ; type B = UInt < UInt < UTerm , B1 > , B1 > ; # [allow (non_camel_case_types)] type U1CmpU3 = < A as Cmp < B > > :: Output ; assert_eq ! (< U1CmpU3 as Ord >:: to_ordering () , Ordering :: Less) ; }
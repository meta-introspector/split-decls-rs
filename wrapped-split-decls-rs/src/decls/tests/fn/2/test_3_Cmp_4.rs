use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_3_Cmp_4 () { type A = UInt < UInt < UTerm , B1 > , B1 > ; type B = UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > ; # [allow (non_camel_case_types)] type U3CmpU4 = < A as Cmp < B > > :: Output ; assert_eq ! (< U3CmpU4 as Ord >:: to_ordering () , Ordering :: Less) ; }
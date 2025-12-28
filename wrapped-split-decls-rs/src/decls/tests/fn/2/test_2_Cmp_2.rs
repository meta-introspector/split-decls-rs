use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_2_Cmp_2 () { type A = UInt < UInt < UTerm , B1 > , B0 > ; type B = UInt < UInt < UTerm , B1 > , B0 > ; # [allow (non_camel_case_types)] type U2CmpU2 = < A as Cmp < B > > :: Output ; assert_eq ! (< U2CmpU2 as Ord >:: to_ordering () , Ordering :: Equal) ; }